use derive_builder::Builder;
use reqwest::{Client, Url, Response};
use scraper::{Html};

use crate::{constant::BASE_URL, types::{home::{RawKumaHomeData, RawKumaHomeDataBuilder}, RawKumaResult}, handle_reqwest_error, handle_rawkuma_result, parser::home::RawKumaHomeParser, handle_other_error};

#[derive(Clone, Builder)]
pub struct RawKumaClient{
    http_client : Client,
    api_url : Url
}

impl Default for RawKumaClient {
    fn default() -> Self {
        Self { http_client: Client::new(), api_url: Url::parse(BASE_URL).expect("Error on parsing the BASE_URL") }
    }
}

impl RawKumaClient {
    pub fn new(client : Client) -> Self {
        Self { http_client: client, api_url: Url::parse(BASE_URL).expect("Error on parsing the BASE_URL") }
    }
    async fn send(&mut self, url : Url) -> RawKumaResult<Response>{
        let req = handle_reqwest_error!(self.http_client.get(url).build());
        let res = handle_reqwest_error!(self.http_client.execute(req).await);
        return RawKumaResult::Ok(res);
    }
    pub async fn home(&mut self) -> RawKumaResult<RawKumaHomeData>{
        let res = handle_rawkuma_result!(self.send(self.api_url.clone()).await);
        let html = Html::parse_document(handle_reqwest_error!(res.text().await).as_str());
        let parser_result = RawKumaHomeParser::init(&html);
        let home = match parser_result {
            RawKumaResult::Ok(d) => d,
            RawKumaResult::ReqwestError(err) => return RawKumaResult::ReqwestError(err),
            RawKumaResult::Other(err) => return RawKumaResult::Other(err),
            RawKumaResult::Io(err) => return RawKumaResult::Io(err)
        };
        let data = handle_other_error!(RawKumaHomeDataBuilder::default()
            .popular_title(home.get_popular_today())
            .recommandation(home.get_recommandation())
            .latest_update(home.get_latest())
            .build()
        );
        RawKumaResult::Ok(data)
    }
}