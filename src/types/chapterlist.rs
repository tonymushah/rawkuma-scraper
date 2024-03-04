use derive_builder::Builder;
use reqwest::Url;
use scraper::{ElementRef, Selector};

use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use super::{error, FromElementRef, RawKumaResult};

#[derive(Builder, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct ChapterList {
    pub chapters: Vec<Chapter>,
}

impl<'a> ChapterList {
    pub fn get_chapter_list_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div#chapterlist"#)?)
    }
    pub fn get_chapter_list_element(data: &'a ElementRef) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_chapter_list_selector()?;
        data.select(&selector)
            .next()
            .ok_or(super::error::Error::ElementNotFound(
                r#"div#chapterlist"#.to_string(),
            ))
    }
    pub fn get_chapters_elements(data: &'a ElementRef) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = Chapter::get_data_num_selector()?;
        RawKumaResult::Ok(data.select(&selector).collect())
    }
}

impl<'a> FromElementRef<'a> for ChapterList {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(
            ChapterListBuilder::default()
                .chapters(Chapter::from_vec_element(&Self::get_chapters_elements(
                    data,
                )?)?)
                .build()?,
        )
    }
}

#[derive(Builder, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct Chapter {
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub url: Url,
    pub chapter_num: String,
    pub chapter_date: String,
    pub num: f32,
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub download_link: Url,
}

impl<'a> Chapter {
    pub fn get_data_num_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse("li[data-num]")?)
    }
    pub fn get_eph_num_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse("div.eph-num")?)
    }
    pub fn get_dload_a_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse("a.dload")?)
    }
    fn get_a_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse("a")?)
    }
    fn get_chapternum_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(".chapternum")?)
    }
    fn get_chapterdate_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(".chapterdate")?)
    }

    pub fn get_eph_num_element(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_eph_num_selector()?;
        data.select(&selector)
            .next()
            .ok_or(super::error::Error::ElementNotFound(
                "div.eph-num".to_string(),
            ))
    }
    pub fn get_dload_element(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_dload_a_selector()?;
        data.select(&selector)
            .next()
            .ok_or(super::error::Error::ElementNotFound("a.dload".to_string()))
    }
    pub fn get_a_ephnum_element(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_a_selector()?;
        let ephnum = Self::get_eph_num_element(data)?;
        ephnum
            .select(&selector)
            .next()
            .ok_or(super::error::Error::ElementNotFoundInNested {
                element: "a".to_string(),
                parent: "div.eph-num".to_string(),
            })
    }
    pub fn get_chapterdate_element(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_chapterdate_selector()?;
        let a_ephnum = Self::get_a_ephnum_element(data)?;
        a_ephnum
            .select(&selector)
            .next()
            .ok_or(super::error::Error::ElementNotFoundInNested {
                element: ".chapternum".to_string(),
                parent: "a > .eph-num".to_string(),
            })
    }
    pub fn get_chapternum_element(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_chapternum_selector()?;
        let a_ephnum = Self::get_a_ephnum_element(data)?;
        a_ephnum
            .select(&selector)
            .next()
            .ok_or(super::error::Error::ElementNotFoundInNested {
                element: ".chapternum".to_string(),
                parent: "a > .eph-num".to_string(),
            })
    }

    pub fn get_a_ephnum_data(data: &'a ElementRef<'a>) -> RawKumaResult<Url> {
        let a_ephnum = Self::get_a_ephnum_element(data)?;
        Ok(Url::parse(a_ephnum.value().attr("href").ok_or(
            super::error::Error::AttributeNotFound {
                name: "href".to_string(),
                element: "a > .eph-num".to_string(),
            },
        )?)?)
    }
    pub fn get_chapternum_data(data: &'a ElementRef<'a>) -> RawKumaResult<String> {
        let chapternum = Self::get_chapternum_element(data)?;
        Ok(chapternum
            .text()
            .next()
            .map(|d| d.to_string())
            .unwrap_or(String::new()))
    }
    pub fn get_chapterdate_data(data: &'a ElementRef<'a>) -> RawKumaResult<String> {
        let chapterdate = Self::get_chapterdate_element(data)?;
        Ok(chapterdate
            .text()
            .next()
            .map(|d| d.to_string())
            .unwrap_or(String::new()))
    }
    pub fn get_data_num(data: &'a ElementRef<'a>) -> RawKumaResult<f32> {
        Ok(data
            .value()
            .attr("data-num")
            .ok_or(error::Error::AttributeNotFound {
                name: "data-num".to_string(),
                element: data.html(),
            })?
            .parse::<f32>()?)
    }
    pub fn get_dload_data(data: &'a ElementRef<'a>) -> RawKumaResult<Url> {
        let element = Self::get_dload_element(data)?;
        Ok(element
            .value()
            .attr("href")
            .ok_or(error::Error::AttributeNotFound {
                name: "href".to_string(),
                element: "a.dload".to_string(),
            })?
            .parse::<Url>()?)
    }
}

impl<'a> FromElementRef<'a> for Chapter {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(
            ChapterBuilder::default()
                .chapter_date(Self::get_chapterdate_data(data)?)
                .chapter_num(Self::get_chapternum_data(data)?)
                .download_link(Self::get_dload_data(data)?)
                .num(Self::get_data_num(data)?)
                .url(Self::get_a_ephnum_data(data)?)
                .build()?,
        )
    }
}
