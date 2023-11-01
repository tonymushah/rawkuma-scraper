use jsonxf::pretty_print;
use rawkuma_scraper::RawKumaClient;

#[tokio::main]
async fn main() {
    let mut client = RawKumaClient::default();
    let data = client.search(&"").await.unwrap();
    println!(
        "{}",
        pretty_print(serde_json::to_string(&(data)).unwrap().as_str()).unwrap()
    );
}
