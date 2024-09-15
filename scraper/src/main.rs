mod scraper_client;
mod supabase_client;
mod constants;

use tokio;
use scraper_client::ScraperClient;
use supabase_client::SupabaseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let supabase_client = SupabaseClient::new()?;
    supabase_client.ping().await?;

    let sites = vec![
        "greenhouse.io",
        // "lever.co",
        // "ashbyhq.com",
        // "paylocity.com",
        // "workable.com",
        // "icims.com",
        // "myworkdayjobs.com",
        // "jobvite.com",
        // "breezy.hr",
        // "jobs.smartrecruiters.com/",
    ];

    let mut scraper = ScraperClient::new("https://www.google.com/search", r#"allintitle: Intern"#, "qdr:d", sites.clone(), &supabase_client)?;
    scraper.run_scraper().await?;

    println!("Scraping and processing completed.");
    Ok(())
}