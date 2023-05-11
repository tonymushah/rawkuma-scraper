use derive_builder::Builder;
use reqwest::{Client, Response, Url};
use scraper::Html;

use crate::{
    constant::BASE_URL,
    handle_other_error, handle_rawkuma_result, handle_reqwest_error,
    parser::{home::RawKumaHomeParser, HtmlParser},
    types::{
        home::{RawKumaHomeData, RawKumaHomeDataBuilder},
        RawKumaResult, FromHtmlParser,
    },
};

#[derive(Clone, Builder)]
pub struct RawKumaClient {
    http_client: Client,
    api_url: Url,
}

impl Default for RawKumaClient {
    fn default() -> Self {
        Self {
            http_client: Client::new(),
            api_url: Url::parse(BASE_URL).expect("Error on parsing the BASE_URL"),
        }
    }
}

impl RawKumaClient {
    pub fn new(client: Client) -> Self {
        Self {
            http_client: client,
            api_url: Url::parse(BASE_URL).expect("Error on parsing the BASE_URL"),
        }
    }
    async fn send(&mut self, url: Url) -> RawKumaResult<Response> {
        let req = handle_reqwest_error!(self.http_client.get(url).build());
        let res = handle_reqwest_error!(self.http_client.execute(req).await);
        return RawKumaResult::Ok(res);
    }
    pub async fn home(&mut self) -> RawKumaResult<RawKumaHomeData> {
        let res = handle_rawkuma_result!(self.send(self.api_url.clone()).await);
        let html = Html::parse_document(handle_reqwest_error!(res.text().await).as_str());
        let home = handle_rawkuma_result!(RawKumaHomeParser::init(&html));
        RawKumaResult::Ok(handle_rawkuma_result!(<RawKumaHomeData as FromHtmlParser<RawKumaHomeParser>>::from(home)))
    }
}
