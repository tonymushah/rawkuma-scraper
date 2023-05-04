use std::{fs::File, io::Write};

use rawkuma_scraper::RawKumaClient;

#[tokio::main]
async fn main() {
    let mut client = RawKumaClient::default();
    let home = client.home().await.unwrap();
    let mut file_ = File::create("tests/test_data.json").unwrap();
    file_.write(format!("{}", serde_json::to_string(&(home)).unwrap()).as_bytes()).unwrap();
}
