use postgrest::Postgrest;
use dotenv::dotenv;
use serde_json::json;
use chrono::{DateTime, Utc}; 
use std::time::SystemTime;

pub struct SupabaseClient {
    client: Postgrest,
}

impl SupabaseClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Loading Supabase client...");
        dotenv().ok();
        
        let public_key = dotenv::var("SUPABASE_JOBS_PUBLIC_KEY")?;
        let service_key = dotenv::var("SUPABASE_JOBS_SERVICE_KEY")?;
        let base_url = dotenv::var("SUPABASE_JOBS_URL")?;

        let client = Postgrest::new(base_url)
            .insert_header("apikey", public_key)
            .insert_header("Authorization", format!("Bearer {}", service_key));

        Ok(Self { client })
    }

    pub async fn ping(&self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.from("ping").select("*").execute().await?;
        
        if response.status().is_success() {
            println!("Ping to DB successful!");
            Ok(())
        } else {
            println!("Ping failed with status: {}", response.status());
            Err("Ping failed".into())
        }
    }

    pub async fn insert(&self, id: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let record = json!([{
            "id": id,
            "data": data,
            "updated": now
        }]);

        let response = self.client
            .from("unstructured")
            .upsert(record.to_string().as_str())
            .execute()
            .await?;
        
        if response.status().is_success() {
            println!("Data inserted successfully!");
            Ok(())
        } else {
            println!("Data insertion failed with status: {}", response.status());
            Err("Data insertion failed".into())
        }
    }
}