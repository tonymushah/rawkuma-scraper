use reqwest::Client;

/// we use this `rouhou-ore-no-iinazuke-ni-natta-jimiko-ie-de-wa-kawaii-shika-nai-chapter-14/` for testing
#[tokio::main]
async fn main() {
    let request_client = Client::new();
    let mut client = rawkuma_scraper::RawKumaClient::new(request_client);
    let chapter = client
        .chapter(&"rouhou-ore-no-iinazuke-ni-natta-jimiko-ie-de-wa-kawaii-shika-nai-chapter-21-1")
        .await
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&chapter).unwrap());
}
