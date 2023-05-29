use derive_builder::Builder;
use reqwest::Url;
use scraper::{ElementRef, Selector};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "getset")]
use getset::{Getters};

use crate::{handle_other_error, handle_selector_error, handle_rawkuma_result};

use super::{FromElementRef, RawKumaResult};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Builder, Clone)]
pub struct MgenTag{
    pub url : Url,
    pub name : String
}

impl<'a> MgenTag {
    pub fn get_mgen_selector() -> RawKumaResult<Selector>{
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(r#".mgen"#)))
    }
    pub fn get_tag_selector() -> RawKumaResult<Selector>{
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(r#"a[rel="tag"]"#)))
    }
    pub fn get_mgen_element(data : &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>>{
        let selector = handle_rawkuma_result!(Self::get_mgen_selector());
        match data.select(&selector).next() {
            None => RawKumaResult::Io(std::io::Error::new(std::io::ErrorKind::NotFound, ".mgen element not found")),
            Some(d) => RawKumaResult::Ok(d)
        }
    }
    pub fn get_tag_elements(data : &'a ElementRef<'a>) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = handle_rawkuma_result!(Self::get_tag_selector());
        let elements : Vec<ElementRef<'a>> = handle_rawkuma_result!(Self::get_mgen_element(&data)).select(&selector).collect();
        RawKumaResult::Ok(elements)
    }
    pub fn get_tags_elements_data(data : &'a ElementRef<'a>) -> RawKumaResult<Vec<Self>> {
        let elements = handle_rawkuma_result!(Self::get_tag_elements(data));
        Self::from_vec_element(elements)
    }
}

impl<'a> FromElementRef<'a> for MgenTag {
    fn from_element_ref(data: ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized {
        let url = match data.value().attr("href") {
            Some(d) => handle_other_error!(Url::parse(d)),
            None => return RawKumaResult::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "href attribute not found"))
        };
        let name = match data.text().next() {
            Some(d) => d.to_string(),
            None => return RawKumaResult::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Text not found"))
        };
        RawKumaResult::Ok(handle_other_error!(MgenTagBuilder::default().name(name).url(url).build()))
    }
}