use reqwest;
use regex::Regex;
use scraper::{Html, Selector};
use crate::constants::rules::{JobRule, JOB_RULES, UUID_PATTERN};
use url::Url;


pub struct JobScraper {
    base_url: String,
    query: String,
    tbs: String,
    client: reqwest::Client,
}

impl JobScraper {
    pub fn new(base_url: &str, query: &str, site_filter: &str, tbs: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()?;

        let full_query = format!("{} site:{}", query, site_filter);

        Ok(Self {
            base_url: base_url.to_string(),
            query: full_query,
            tbs: tbs.to_string(),
            client,
        })
    }

    fn matches_rule(href: &str, rule: &JobRule) -> bool {
        if !href.contains(rule.site) {
            return false;
        }

        for pattern in rule.patterns {
            if *pattern == UUID_PATTERN {
                let uuid_regex = Regex::new(UUID_PATTERN).unwrap();
                if uuid_regex.is_match(href) {
                    return true;
                }
            } else if href.contains(pattern) {
                return true;
            }
        }

        rule.patterns.is_empty()
    }

    pub async fn scrape(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut start = 0;
        let mut job_links: Vec<String> = Vec::new();
        let mut has_next_page = true;

        while has_next_page {
            let params = vec![
                ("q".to_string(), self.query.clone()),
                ("tbs".to_string(), self.tbs.clone()),
                ("start".to_string(), start.to_string()),
            ];

            let url = Url::parse_with_params(&self.base_url, &params)?;
            println!("Fetching page: {}", url);

            let resp = self.client.get(url).send().await?.text().await?;
            let document = Html::parse_document(&resp);
            let link_selector = Selector::parse("a").unwrap();
            let next_selector = Selector::parse("a#pnnext").unwrap();

            let mut job_links_found = 0;
            for element in document.select(&link_selector) {
                if let Some(href) = element.value().attr("href") {
                    if JOB_RULES.iter().any(|rule| Self::matches_rule(href, rule)) {
                        job_links_found += 1;
                        job_links.push(href.to_string());
                    }
                }
            }

            println!("Found {} job links on this page", job_links_found);

            let next_elements: Vec<_> = document.select(&next_selector).collect();
            has_next_page = !next_elements.is_empty();
            if has_next_page {
                start += 10;
                println!("Moving to next page...");
            } else {
                println!("No more pages to process.");
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }

        Ok(job_links)
    }
}