use reqwest;
use regex;
use scraper::{Html, Selector};
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

        // Construct the full query including the site filter
        let full_query = format!("{} site:{}", query, site_filter);

        Ok(Self {
            base_url: base_url.to_string(),
            query: full_query,
            tbs: tbs.to_string(),
            client,
        })
    }

    pub fn is_uuid_in_href(href: &str) -> bool {
        let uuid_regex = regex::Regex::new(r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b").unwrap();
        
        uuid_regex.is_match(href)
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
                    // Greenhouse.io job links contain "/jobs/" or "/job_app" in the URL
                    if href.contains("greenhouse.io") && (href.contains("/jobs/") || href.contains("/job_app")) {
                        job_links_found += 1;
                        job_links.push(href.to_string());
                    // Lever.co and AshbyHQ job links contain a UUID in the URL, so we can use a regex to match them
                    } else if href.contains("lever.co") || href.contains("ashbyhq.com") && Self::is_uuid_in_href(href) {
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