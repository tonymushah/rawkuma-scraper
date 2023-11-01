use jsonxf::pretty_print;
use rawkuma_scraper::RawKumaClient;

#[tokio::main]
async fn main() {
    let mut client = RawKumaClient::default();
    let home = client.search(&"konsei").await.unwrap();
    println!(
        "{}",
        pretty_print(serde_json::to_string(&(home)).unwrap().as_str()).unwrap()
    );
}
