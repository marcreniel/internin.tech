use postgrest::Postgrest;

pub struct SupabaseClient {
    base_url: String,
    api_key: String,
    client: Postgrest,
}

impl SupabaseClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Postgrest::new(&base_url);
        SupabaseClient {
            base_url,
            api_key,
            client,
        }
    }
}