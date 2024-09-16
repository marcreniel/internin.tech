use reqwest;
use regex::Regex;
use scraper::{Html, Selector};
use crate::constants::rules::{JobRule, JOB_RULES, UUID_PATTERN};
use crate::SupabaseClient;
use url::Url;
use futures::future::join_all;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};
use serde_json::{Value, json};
use chrono::{Local, NaiveDateTime};
use fantoccini::{ClientBuilder};
use tokio;

pub struct ScraperClient<'a> {
    base_url: String,
    query: String,
    tbs: String,
    client: reqwest::Client,
    sites: Vec<&'static str>,
    supabase_client: &'a SupabaseClient,
}

impl<'a> ScraperClient<'a> {
    pub fn new(base_url: &str, query: &str, tbs: &str, sites: Vec<&'static str>, supabase_client: &'a SupabaseClient) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()?;
        
        Ok(Self {
            base_url: base_url.to_string(),
            query: query.to_string(),
            tbs: tbs.to_string(),
            client,
            sites,
            supabase_client,
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

    async fn write_scraped_to_links(&self, scraped_links: Vec<Value>, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(output_file_path)?;
        let mut writer = std::io::BufWriter::new(output_file);

        for scraped_link in scraped_links {
            writeln!(writer, "{}", serde_json::to_string(&scraped_link)?)?;
        }

        writer.flush()?;
        Ok(())
    }

    async fn initialize_scrape(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_dir_all("output")?;
        std::fs::create_dir_all("output")?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let file_path = format!("output/links_{}.json", timestamp);

        let mut tasks = Vec::new();

        for &site in &self.sites {
            let task = self.scrape_site(site, file_path.clone());
            tasks.push(task);
        }

        join_all(tasks).await;

        Ok(())
    }

    async fn scrape_site(&self, site: &'static str, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut start = 0;
        let mut has_next_page = true;
        let mut scraped_links = Vec::new();

        while has_next_page {
            let params = vec![
                ("q".to_string(), format!("{} site:{}", self.query, site)),
                ("tbs".to_string(), self.tbs.clone()),
                ("start".to_string(), start.to_string()),
            ];

            let url = Url::parse_with_params(&self.base_url, &params)?;
            println!("Fetching page: {} for site: {}", url, site);

            let resp = self.client.get(url).send().await?.text().await?;
            let document = Html::parse_document(&resp);

            let link_selector = Selector::parse("a").unwrap();
            let next_selector = Selector::parse("a#pnnext").unwrap();

            let mut job_links_found = 0;

            for element in document.select(&link_selector) {
                if let Some(href) = element.value().attr("href") {
                    if JOB_RULES.iter().any(|rule| rule.site == site && Self::matches_rule(href, rule)) {
                        job_links_found += 1;
                        let json_link = json!({ "link": href });
                        scraped_links.push(json_link);
                    }
                }
            }

            println!("Found {} job links on this page for site: {}", job_links_found, site);

            let next_elements: Vec<_> = document.select(&next_selector).collect();
            has_next_page = !next_elements.is_empty();

            if scraped_links.len() >= 10 || !has_next_page {
                self.write_scraped_to_links(scraped_links.clone(), &file_path).await?;
                scraped_links.clear();
            }

            if has_next_page {
                start += 10;
                println!("Moving to next page...");
            } else {
                println!("No more pages to process for site: {}", site);
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }

        Ok(())
    }

    async fn process_links(&self) -> Result<(), Box<dyn std::error::Error>> {
        let latest_input_file = std::fs::read_dir("output")?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.file_name()?.to_str()?.starts_with("links_") && path.extension()? == "json" {
                    Some(path)
                } else {
                    None
                }
            })
            .max_by_key(|path| {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let date_time_str = &file_name[6..21];
                NaiveDateTime::parse_from_str(date_time_str, "%Y%m%d_%H%M%S").unwrap()
            });
    
        if let Some(input_file_path) = latest_input_file {
            println!("Processing the latest file: {:?}", input_file_path);
            let input_file = File::open(&input_file_path)?;
            let reader = BufReader::new(input_file);
    
            for line in reader.lines() {
                let line = line?;
                let json: Value = serde_json::from_str(&line)?;
                if let Some(link) = json["link"].as_str() {
                    let processed_link = link.trim_end_matches("/apply").to_string();
                    println!("Fetching content for: {}", processed_link);

                    let html_content = self.fetch_content(&processed_link).await?;
                    let document = Html::parse_document(&html_content);
                    let selector = Selector::parse("body > *:not(script):not(style):not(form)").unwrap();
                    let mut text_content: String = document.select(&selector)
                        .flat_map(|element| element.text())
                        .collect::<Vec<_>>()
                        .join(" ");
                    text_content = text_content.split_whitespace().collect::<Vec<_>>().join(" ");
                    let apply_regex = Regex::new(r"(?i)Apply for this job.*$").unwrap();
                    text_content = apply_regex.replace(&text_content, "").to_string();
                    let entity_regex = Regex::new(r"&[a-zA-Z]+;").unwrap();
                    text_content = entity_regex.replace_all(&text_content, "").to_string();
    
                    self.supabase_client.insert(&processed_link, &text_content).await?;
                }
            }
        } else {
            println!("No links_.json files found in the output directory.");
        }
        Ok(())
    }

    async fn fetch_content(&self, url: &str) -> Result<String, fantoccini::error::CmdError> {
        let client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .expect("failed to connect to WebDriver");

        client.goto(url).await?;
        let content = client.source().await?;
        client.close().await?;

        Ok(content)
    }

    pub async fn run_scraper(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting Google scraping...");
        self.initialize_scrape().await?;
        println!("Google scraping completed. Processing links...");
        self.process_links().await?;
        println!("Link processing completed.");
        Ok(())
    }
}