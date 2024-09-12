pub mod job_scraper;
pub mod constants;

use job_scraper::JobScraper;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scraper = JobScraper::new("https://www.google.com/search", "Software Engineer Intern", "icims.com", "qdr:m")?;
    let job_links = scraper.scrape().await?;
    
    // Print out or process the job links as needed
    for link in job_links {
        println!("{}", link);
    }

    Ok(())
}