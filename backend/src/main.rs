mod job_scraper;
mod constants;

use job_scraper::JobScraper;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sites = vec![
        "greenhouse.io",
    ];

    let scraper = JobScraper::new("https://www.google.com/search", r#"allintitle: Intern"#, "qdr:d", sites.clone())?;
    scraper.run_scraper().await?;

    println!("Scraping and processing completed.");
    println!("Check output/links.json for the initial scraped links.");
    println!("Check output/processed.json for the processed page contents.");

    Ok(())
}
