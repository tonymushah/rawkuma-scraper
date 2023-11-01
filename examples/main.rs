use rawkuma_scraper::types::{manga::MangaListParameter, ToUrlParam};
use reqwest::Url;

#[tokio::main]
async fn main() {
    println!(
        "{}",
        Url::parse_with_params(
            "https://rawkuma.com",
            MangaListParameter::default().to_url_param()
        )
        .unwrap()
    );
}
