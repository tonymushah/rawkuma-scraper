use async_trait::async_trait;
use derive_builder::Builder;
use reqwest::{Client, Response, Url};
use scraper::Html;

use crate::{
    constant::BASE_URL,
    parser::{
        chapter::RawKumaChapterParser, home::RawKumaHomeParser,
        manga_details::RawKumaMangaDetailParser, search::RawKumaSearchParser, HtmlParser,
    },
    types::{
        chapter::RawKumaChapterData, home::RawKumaHomeData, manga::RawKumaMangaDetailData,
        search::RawKumaSearch, FromHtmlParser, RawKumaResult,
    },
};

#[derive(Clone, Builder)]
pub struct RawKumaClient {
    http_client: Client,
    api_url: Url,
}

#[async_trait]
pub trait RawKumaClientFromUrl {
    async fn manga_details(&mut self, url: Url) -> RawKumaResult<RawKumaMangaDetailData>;
    async fn chapter(&mut self, url: Url) -> RawKumaResult<RawKumaChapterData>;
    async fn home(&mut self, url: Url) -> RawKumaResult<RawKumaHomeData>;
    async fn search(&mut self, url: Url) -> RawKumaResult<RawKumaSearch>;
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
    async fn manga_details(&mut self, url: Url) -> RawKumaResult<RawKumaMangaDetailData> {
        type Output = RawKumaMangaDetailData;
        let res = self.send_get(url).await?;
        let html = Html::parse_document(res.text().await?.as_str());
        let parser = RawKumaMangaDetailParser::init(&html)?;
        RawKumaResult::Ok(<Output as FromHtmlParser<RawKumaMangaDetailParser>>::from(
            parser,
        )?)
    }
    async fn chapter(&mut self, url: Url) -> RawKumaResult<RawKumaChapterData> {
        type Output = RawKumaChapterData;
        let res = self.send_get(url).await?;
        let html = Html::parse_document(res.text().await?.as_str());
        let parser = RawKumaChapterParser::init(&html)?;
        RawKumaResult::Ok(<Output as FromHtmlParser<RawKumaChapterParser>>::from(
            parser,
        )?)
    }
    async fn home(&mut self, url: Url) -> RawKumaResult<RawKumaHomeData> {
        type Output = RawKumaHomeData;
        let res = self.send_get(url).await?;
        let html = Html::parse_document(res.text().await?.as_str());
        let home = RawKumaHomeParser::init(&html)?;
        RawKumaResult::Ok(<Output as FromHtmlParser<RawKumaHomeParser>>::from(home)?)
    }
    async fn search(&mut self, url: Url) -> RawKumaResult<RawKumaSearch> {
        type Output = RawKumaSearch;
        let res = self.send_get(url).await?;
        let html = Html::parse_document(res.text().await?.as_str());
        let home = RawKumaSearchParser::init(&html)?;
        RawKumaResult::Ok(<Output as FromHtmlParser<RawKumaSearchParser>>::from(home)?)
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
        let req = self.http_client.get(url).build()?;
        let res = self.http_client.execute(req).await?;
        RawKumaResult::Ok(res)
    }
    pub async fn home(&mut self) -> RawKumaResult<RawKumaHomeData> {
        let url = self.api_url.clone();
        RawKumaClientFromUrl::home(self, url).await
    }
    pub async fn manga_details(
        &mut self,
        manga_slug: &dyn ToString,
    ) -> RawKumaResult<RawKumaMangaDetailData> {
        let url = Url::parse(format!("{}{}", self.api_url, manga_slug.to_string()).as_str())?;
        RawKumaClientFromUrl::manga_details(self, url).await
    }
    pub async fn chapter(
        &mut self,
        chapter_slug: &dyn ToString,
    ) -> RawKumaResult<RawKumaChapterData> {
        let url = Url::parse(format!("{}{}", self.api_url, chapter_slug.to_string()).as_str())?;
        RawKumaClientFromUrl::chapter(self, url).await
    }
    pub async fn search(&mut self, search_query: &dyn ToString) -> RawKumaResult<RawKumaSearch> {
        let url = Url::parse_with_params(
            self.api_url.to_string().as_str(),
            [("s", search_query.to_string().as_str())],
        )?;
        RawKumaClientFromUrl::search(self, url).await
    }
}
