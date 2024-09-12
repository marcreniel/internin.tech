use reqwest;
use scraper::{Html, Selector};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://www.google.com/search";
    let query = "Software Engineer Intern site:greenhouse.io";
    let mut start = 0;
    let mut job_links: Vec<String> = Vec::new();
    let mut has_next_page = true;

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

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

        let resp = client.get(url).send().await?.text().await?;
        let document = Html::parse_document(&resp);

        let link_selector = Selector::parse("a").unwrap();
        let next_selector = Selector::parse("a#pnnext").unwrap();

        println!("Searching for job links...");
        let mut job_links_found = 0;
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if href.contains("greenhouse.io") && (href.contains("/jobs/") || href.contains("/job_app")) {
                    job_links_found += 1;
                    job_links.push(href.to_string());
                }
            }
        }
        println!("Found {} job links on this page", job_links_found);

        println!("Searching for next button...");
        let next_elements: Vec<_> = document.select(&next_selector).collect();
        println!("Found {} elements matching the next selector", next_elements.len());   
        
        has_next_page = !next_elements.is_empty();
        if has_next_page {
            start += 10; 
            println!("Moving to next page...");
        } else {
            println!("No more pages to process.");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    Ok(())
}