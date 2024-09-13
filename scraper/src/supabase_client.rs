use postgrest::Postgrest;
use dotenv::dotenv;

pub struct SupabaseClient {
    client: Postgrest,
}

impl SupabaseClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        
        let public_key = dotenv::var("SUPABASE_JOBS_PUBLIC_KEY")?;
        let service_key = dotenv::var("SUPABASE_JOBS_SERVICE_KEY")?;
        let base_url = dotenv::var("SUPABASE_JOBS_URL")?;
        
        let client = Postgrest::new(base_url)
        .insert_header(
            "apikey",
            public_key)
        .insert_header("Authorization", format!("Bearer {}", service_key));

        Ok(Self {
            client,
        })
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
}