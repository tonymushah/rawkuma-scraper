use derive_builder::Builder;
use reqwest::Url;
use scraper::{ElementRef, Selector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use super::{error::Error, FromElementRef, RawKumaResult};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[derive(Builder, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct UtaoTitleChapter {
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub url: Url,
    pub text: String,
}

impl<'a> UtaoTitleChapter {
    pub fn get_a_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse(r#"a"#)?)
    }
    pub fn get_title_selector(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        data.select(&(Self::get_a_selector()?))
            .next()
            .ok_or(Error::ElementNotFound("a".to_string()))
    }
}

impl<'a> FromElementRef<'a> for UtaoTitleChapter {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self> {
        let title = Self::get_title_selector(data)?;
        RawKumaResult::Ok(
            UtaoTitleChapterBuilder::default()
                .url(
                    title
                        .value()
                        .attr("href")
                        .ok_or(Error::AttributeNotFound {
                            name: "href".to_string(),
                            element: "a".to_string(),
                        })?
                        .parse::<Url>()?,
                )
                .text(
                    title
                        .text()
                        .next()
                        .ok_or(Error::TextContentFound)?
                        .to_string(),
                )
                .build()?,
        )
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Builder, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct UtaoTitleData {
    pub title: String,
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub url: Url,
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub image: Url,
    pub chapters: Vec<UtaoTitleChapter>,
}

impl<'a> UtaoTitleData {
    pub fn div_imagu_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[class="imgu"]"#)?)
    }

    pub fn get_imgu_div(html: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let divs: ElementRef = html
            .select(&(Self::div_imagu_selector()?))
            .next()
            .ok_or(Error::ElementNotFound("a".to_string()))?;
        RawKumaResult::Ok(divs)
    }

    pub fn get_img_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse("img")?)
    }

    pub fn get_image_element(imgu: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        imgu.select(&(Self::get_img_selector()?))
            .next()
            .ok_or(Error::ElementNotFound("img".to_string()))
    }

    pub fn get_a_series_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse(r#"a[class="series"]"#)?)
    }

    pub fn get_a_series_element(imgu: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        imgu.select(&(Self::get_a_series_selector()?))
            .next()
            .ok_or(Error::ElementNotFound("a".to_string()))
    }

    pub fn get_ul_manga_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse(r#"ul[class="Manga"]"#)?)
    }

    pub fn get_ul_manga_element(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        data.select(&(Self::get_ul_manga_selector()?))
            .next()
            .ok_or(Error::ElementNotFound(r#"ul[class="Manga"]"#.to_string()))
    }

    pub fn get_li_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse("li")?)
    }

    pub fn get_chapters_elements(data: &'a ElementRef<'a>) -> RawKumaResult<Vec<ElementRef<'a>>> {
        Ok(Self::get_ul_manga_element(data)?
            .select(&(Self::get_li_selector()?))
            .collect())
    }
}

impl<'a> FromElementRef<'a> for UtaoTitleData {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self> {
        let imgu = Self::get_imgu_div(data)?;
        let image = Self::get_image_element(&imgu)?;
        let title = Self::get_a_series_element(&imgu)?;
        let chapters: Vec<ElementRef> = Self::get_chapters_elements(data)?;
        RawKumaResult::Ok(
            UtaoTitleDataBuilder::default()
                .chapters(UtaoTitleChapter::from_vec_element(&chapters)?)
                .image(
                    format!(
                        "https:{}",
                        image.value().attr("src").ok_or(Error::AttributeNotFound {
                            name: "src".to_string(),
                            element: "image".to_string()
                        })?
                    )
                    .as_str()
                    .parse::<Url>()?,
                )
                .url(
                    title
                        .value()
                        .attr("href")
                        .ok_or(Error::AttributeNotFound {
                            name: "href".to_string(),
                            element: "a".to_string(),
                        })?
                        .parse::<Url>()?,
                )
                .title(title.value().attr("title").map(|d| d.to_string()).ok_or(
                    Error::AttributeNotFound {
                        name: "title".to_string(),
                        element: r#"a[class="series"]"#.to_string(),
                    },
                )?)
                .build()?,
        )
    }
}
