#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_okapi;

mod scraper_client;
mod supabase_client;
mod constants;

use scraper_client::ScraperClient;
use supabase_client::SupabaseClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use rocket::State;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[openapi]
#[get("/start-scraping")]
async fn start_scraping(scraper: &State<Arc<Mutex<ScraperClient>>>) -> &'static str {
    let scraper_clone = Arc::clone(scraper);
    tokio::task::spawn(async move {
        let scraper = scraper_clone.lock().await;
        if let Err(e) = scraper.run_scraper().await {
            eprintln!("Error during scraping: {:?}", e);
        }
    });
    "Scraping started, observe logs to ensure completion"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let supabase_client = Arc::new(SupabaseClient::new().expect("Failed to create SupabaseClient"));
    supabase_client.ping().await.expect("Failed to ping Supabase");

    let sites = vec![
        "greenhouse.io".to_string(),
        "lever.co".to_string(),
        "ashbyhq.com".to_string(),
        "paylocity.com".to_string(),
        "workable.com".to_string(),
        "icims.com".to_string(),
        "myworkdayjobs.com".to_string(),
        "jobvite.com".to_string(),
        "breezy.hr".to_string(),
        "jobs.smartrecruiters.com/".to_string(),
    ];

    let scraper = ScraperClient::new(
        "https://www.google.com/search",
        r#"allintitle: Intern"#,
        "qdr:d",
        sites,
        Arc::clone(&supabase_client),
    ).expect("Failed to create ScraperClient");

    rocket::build()
        .manage(Arc::new(Mutex::new(scraper)))
        .mount("/", openapi_get_routes![start_scraping])
        .mount(
            "/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await?;

    Ok(())
}