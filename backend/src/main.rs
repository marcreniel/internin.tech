mod job_scraper;
mod constants;

use job_scraper::JobScraper;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sites = vec![
        "greenhouse.io",
        "lever.co",
        "ashbyhq.com",
        "paylocity.com",
        "workable.com",
        "icims.com",
        "myworkdayjobs.com",
        "jobvite.com",
        "breezy.hr",
        "jobs.smartrecruiters.com/",
    ];

    let scraper = JobScraper::new("https://www.google.com/search", "Software Engineer Intern", "qdr:d", sites)?;
    scraper.scrape().await?;

    println!("Scraping completed. Check output/links.json for the results.");

    Ok(())
}
