use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE};
use scraper::{Html, Selector};
use url::Url;
use std::error::Error;
use tokio::time::{sleep, Duration};
use reqwest::Client;

async fn build_client() -> Result<Client, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9,lt;q=0.8,et;q=0.7,de;q=0.6"));

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}

async fn scrape_jobs(base_url: &str, query: &str) -> Result<(), Box<dyn Error>> {
    let mut start = 0;
    let mut job_links: Vec<String> = Vec::new();
    let mut has_next_page = true;

    let client = build_client().await?;

    while has_next_page {
        let url = Url::parse_with_params(
            base_url,
            &[
                ("q", query),
                ("tbs", "qdr:w"),
                ("start", &start.to_string()),
            ],
        )?;
        println!("Fetching page: {}", url);

        let page_content = client.get(url).send().await?.text().await?;
        let document = Html::parse_document(&page_content);

        println!("Page fetched successfully, {}", page_content);

        let link_selector = Selector::parse("a").unwrap();
        let job_links_found: Vec<String> = document
            .select(&link_selector)
            .filter_map(|element| {
                element.value().attr("href").map(|href| {
                    if href.contains("greenhouse.io") && (href.contains("/jobs/") || href.contains("/job_app")) {
                        href.to_string()
                    } else {
                        String::new()
                    }
                })
            })
            .filter(|href| !href.is_empty())
            .collect();

        println!("Found {} job links on this page", job_links_found.len());
        job_links.extend(job_links_found);

        let next_selector = Selector::parse("#pnnext").unwrap();
        let next_elements: Vec<_> = document.select(&next_selector).collect();
        has_next_page = !next_elements.is_empty();
        if has_next_page {
            start += 10;
            println!("Moving to next page...");
        } else {
            println!("No more pages to process.");
        }

        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://www.google.com/search";
    let query = "Software Engineer Intern site:greenhouse.io";

    scrape_jobs(base_url, query).await?;

    Ok(())
}
