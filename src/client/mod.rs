use async_trait::async_trait;
use derive_builder::Builder;
use reqwest::{Client, Response, Url};
use scraper::Html;

use crate::{
    constant::BASE_URL,
    handle_other_error, handle_rawkuma_result, handle_reqwest_error,
    parser::{home::RawKumaHomeParser, HtmlParser, manga_details::RawKumaMangaDetailParser},
    types::{
        home::{RawKumaHomeData},
        RawKumaResult, FromHtmlParser, manga::RawKumaMangaDetailData,
    },
};

#[derive(Clone, Builder)]
pub struct RawKumaClient {
    http_client: Client,
    api_url: Url,
}

#[async_trait]
pub trait RawKumaClientFromUrl {
    async fn manga_details(&mut self, url : Url) -> RawKumaResult<RawKumaMangaDetailData>;
}

impl Default for RawKumaClient {
    fn default() -> Self {
        Self {
            http_client: Client::new(),
            api_url: Url::parse(BASE_URL).expect("Error on parsing the BASE_URL"),
        }
    }
}

#[async_trait]
impl RawKumaClientFromUrl for RawKumaClient {
    async fn manga_details(&mut self, url : Url) -> RawKumaResult<RawKumaMangaDetailData> {
        type Output = RawKumaMangaDetailData;
        let res = handle_rawkuma_result!(self.send_get(url).await);
        let html = Html::parse_document(handle_reqwest_error!(res.text().await).as_str());
        let parser = handle_rawkuma_result!(RawKumaMangaDetailParser::init(&html));
        RawKumaResult::Ok(handle_rawkuma_result!(<Output as FromHtmlParser<RawKumaMangaDetailParser>>::from(parser)))
    }
}

impl RawKumaClient {
    pub fn new(client: Client) -> Self {
        Self {
            http_client: client,
            api_url: Url::parse(BASE_URL).expect("Error on parsing the BASE_URL"),
        }
    }
    async fn send_get(&mut self, url: Url) -> RawKumaResult<Response> {
        let req = handle_reqwest_error!(self.http_client.get(url).build());
        let res = handle_reqwest_error!(self.http_client.execute(req).await);
        return RawKumaResult::Ok(res);
    }
    pub async fn home(&mut self) -> RawKumaResult<RawKumaHomeData> {
        type Output = RawKumaHomeData;
        let res = handle_rawkuma_result!(self.send_get(self.api_url.clone()).await);
        let html = Html::parse_document(handle_reqwest_error!(res.text().await).as_str());
        let home = handle_rawkuma_result!(RawKumaHomeParser::init(&html));
        RawKumaResult::Ok(handle_rawkuma_result!(<Output as FromHtmlParser<RawKumaHomeParser>>::from(home)))
    }
    pub async fn manga_details(&mut self, manga_slug : &dyn ToString) -> RawKumaResult<RawKumaMangaDetailData> {
        type Output = RawKumaMangaDetailData;
        let url = handle_other_error!(Url::parse(format!("{}{}", self.api_url, manga_slug.to_string()).as_str()));
        let res = handle_rawkuma_result!(self.send_get(url).await);
        let html = Html::parse_document(handle_reqwest_error!(res.text().await).as_str());
        let parser = handle_rawkuma_result!(RawKumaMangaDetailParser::init(&html));
        RawKumaResult::Ok(handle_rawkuma_result!(<Output as FromHtmlParser<RawKumaMangaDetailParser>>::from(parser)))
    }
    
}
