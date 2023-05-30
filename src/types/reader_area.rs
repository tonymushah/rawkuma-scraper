use derive_builder::Builder;
use htmlize::unescape;
use reqwest::Url;
use scraper::{ElementRef, Selector, Html};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "getset")]
use getset::{Getters};

use crate::{handle_other_error, handle_rawkuma_result, handle_selector_error};

use super::{FromElementRef, RawKumaResult};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Builder, Clone, Default)]
pub struct ReaderArea {
    pub images: Vec<ReaderAreaImage>,
}

impl<'a> ReaderArea {
    pub fn get_reader_area_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse("#readerarea")))
    }
    pub fn get_reader_area_element(data: &'a ElementRef) -> RawKumaResult<ElementRef<'a>> {
        let selector = handle_rawkuma_result!(Self::get_reader_area_selector());
        match data.select(&selector).next() {
            None => RawKumaResult::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "#readerarea not found",
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
}

impl<'a> FromElementRef<'a> for ReaderArea {
    fn from_element_ref(data: ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let selector = handle_selector_error!(Selector::parse("noscript"));
        let data = match data.select(&selector).next() {
            None => {
                return RawKumaResult::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "noscript element not found"));
            },
            Some(d) => d
        };
        let d = unescape(data.inner_html());
        let data = Html::parse_fragment(d.to_string().as_str());
        let images_elements =
            handle_rawkuma_result!(ReaderAreaImage::get_reader_area_images_element_from_html(&data));
        let images: Vec<ReaderAreaImage> =
            handle_rawkuma_result!(ReaderAreaImage::from_vec_element(images_elements));
        RawKumaResult::Ok(handle_other_error!(ReaderAreaBuilder::default()
            .images(images)
            .build()))
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Builder, Clone, Debug)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReaderAreaImage {
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub url: Url,
    pub width : f32,
    pub height : f32,
    pub decoding : String,
    pub alt : String
}

impl<'a> ReaderAreaImage {
    pub fn get_reader_area_image_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse("img")))
    }
    #[warn(dead_code)]
    pub fn get_reader_area_images_element(
        data: &'a ElementRef<'a>,
    ) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = handle_rawkuma_result!(Self::get_reader_area_image_selector());
        RawKumaResult::Ok(data.select(&selector).collect())
    }
    pub fn get_reader_area_images_element_from_html(
        data: &'a Html,
    ) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = handle_rawkuma_result!(Self::get_reader_area_image_selector());
        RawKumaResult::Ok(data.select(&selector).collect())
    }
}

impl<'a> FromElementRef<'a> for ReaderAreaImage {
    fn from_element_ref(data: ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let width : f32 = match data.value().attr("width") {
            None => 0.0,
            Some(d) => {
                handle_other_error!(d.parse())
            }
        };
        let height : f32 = match data.value().attr("height") {
            None => 0.0,
            Some(d) => {
                handle_other_error!(d.parse())
            }
        };
        let decoding : String = match data.value().attr("decoding") {
            None => String::new(),
            Some(d) => {
                d.to_string()
            }
        };
        let alt : String = match data.value().attr("alt") {
            None => String::new(),
            Some(d) => {
                d.to_string()
            }
        };
        let url: Url = match data.value().attr("src") {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "src not found",
                ))
            }
            Some(d) => {
                handle_other_error!(Url::parse(format!("http:{}", d).as_str()))
            }
        };
        RawKumaResult::Ok(handle_other_error!(ReaderAreaImageBuilder::default()
            .width(width)
            .height(height)
            .decoding(decoding)
            .alt(alt)
            .url(url)
            .build()))
    }
}
