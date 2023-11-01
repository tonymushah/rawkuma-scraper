use derive_builder::Builder;
use reqwest::{Client, Response, Url};
use scraper::{ElementRef, Selector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use crate::{client::RawKumaClientFromUrl, RawKumaClient};

use super::{manga::RawKumaMangaDetailData, FromElementRef, RawKumaResult};

#[derive(Builder, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct BsxTitleData {
    pub title: String,
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub url: Url,
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub image: Url,
    pub rating: f64,
}

#[derive(Debug, Clone)]
pub struct TitleData {
    pub title: String,
    pub url: Url,
}

impl BsxTitleData {
    pub fn div_bsx_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[class="bsx"]"#)?)
    }
    pub async fn get_url_manga_detail(
        &self,
        client: &mut RawKumaClient,
    ) -> RawKumaResult<RawKumaMangaDetailData> {
        RawKumaClientFromUrl::manga_details(client, self.url.clone()).await
    }
    pub async fn get_image_response(&self, client: Client) -> RawKumaResult<Response> {
        let req = client.get(self.image.clone()).build()?;
        RawKumaResult::Ok(client.execute(req).await?)
    }
    pub fn get_rating_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse(r#"div[class="numscore"]"#)?)
    }
    pub fn get_bsx_elements<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = Self::div_bsx_selector()?;
        RawKumaResult::Ok(data.select(&selector).collect())
    }
    pub fn get_title_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        match data.select(&(Selector::parse(r#"a"#)?)).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound("a".to_string())),
            Some(d) => Ok(d),
        }
    }
    pub fn get_image_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        match data.select(&(Selector::parse("img")?)).next() {
            None => Err(super::error::Error::ElementNotFound("img".to_string())),
            Some(d) => Ok(d),
        }
    }
    pub fn get_rating_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        match data.select(&(Self::get_rating_selector()?)).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"div[class="numscore"]"#.to_string(),
            )),
            Some(d) => Ok(d),
        }
    }
    pub fn get_title_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<TitleData> {
        let _title = Self::get_title_element(data)?;
        Ok(TitleData {
            title: _title
                .value()
                .attr("title")
                .ok_or(super::error::Error::AttributeNotFound {
                    name: "title".to_string(),
                    element: "a".to_string(),
                })?
                .to_string(),
            url: Url::parse(_title.value().attr("href").ok_or(
                super::error::Error::AttributeNotFound {
                    name: "href".to_string(),
                    element: "a".to_string(),
                },
            )?)?,
        })
    }
    pub fn get_img_url<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<Url> {
        let image = Self::get_image_element(data)?;
        Ok(Url::parse(
            format!(
                "https:{}",
                image
                    .value()
                    .attr("src")
                    .ok_or(super::error::Error::AttributeNotFound {
                        name: "href".to_string(),
                        element: "a".to_string(),
                    })?
            )
            .as_str(),
        )?)
    }
    pub fn get_rating<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<f64> {
        let rating = Self::get_rating_element(data)?;
        Ok(rating
            .text()
            .next()
            .ok_or(super::error::Error::TextContentFound)?
            .parse::<f64>()?)
    }
}

impl<'a> FromElementRef<'a> for BsxTitleData {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self> {
        let title = Self::get_title_data(data)?;

        RawKumaResult::Ok(
            BsxTitleDataBuilder::default()
                .title(title.title)
                .rating(Self::get_rating(data)?)
                .image(Self::get_img_url(data)?)
                .url(title.url)
                .build()?,
        )
    }
}
