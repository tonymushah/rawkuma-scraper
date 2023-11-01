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
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Builder, Clone)]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct MgenTag {
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub url: Url,
    pub name: String,
}

impl<'a> MgenTag {
    pub fn get_mgen_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#".mgen"#)?)
    }
    pub fn get_tag_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"a[rel="tag"]"#)?)
    }
    pub fn get_mgen_element(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_mgen_selector()?;
        data.select(&selector)
            .next()
            .ok_or(Error::ElementNotFound(".mgen".to_string()))
    }
    pub fn get_tag_elements(data: &'a ElementRef<'a>) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = Self::get_tag_selector()?;
        let elements: Vec<ElementRef<'a>> =
            Self::get_mgen_element(&data)?.select(&selector).collect();
        RawKumaResult::Ok(elements)
    }
    pub fn get_tags_elements_data(data: &'a ElementRef<'a>) -> RawKumaResult<Vec<Self>> {
        let elements = Self::get_tag_elements(data)?;
        Self::from_vec_element(elements)
    }
}

impl<'a> FromElementRef<'a> for MgenTag {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let url = data
            .value()
            .attr("href")
            .ok_or(Error::AttributeNotFound {
                name: "href".to_string(),
                element: "a".to_string(),
            })?
            .parse::<Url>()?;
        let name = data
            .text()
            .next()
            .map(|d| d.to_string())
            .ok_or(Error::TextContentFound)?;
        RawKumaResult::Ok(MgenTagBuilder::default().name(name).url(url).build()?)
    }
}
