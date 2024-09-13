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
    ];

    let mut scraper = ScraperClient::new("https://www.google.com/search", r#"allintitle: Intern"#, "qdr:d", sites.clone())?;
    scraper.run_scraper().await?;

    println!("Scraping and processing completed.");
    println!("Check output/links.json for the initial scraped links.");
    println!("Check output/processed.json for the processed page contents.");
    
    Ok(())
}