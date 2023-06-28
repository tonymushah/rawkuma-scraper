use derive_builder::Builder;
use reqwest::Url;
use scraper::{ElementRef, Selector};
use serde::Serialize;

use crate::{handle_other_error, handle_rawkuma_result, handle_selector_error};

use super::{FromElementRef, RawKumaResult};

#[derive(Builder, Clone, Serialize)]
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
        let images_elements =
            handle_rawkuma_result!(ReaderAreaImage::get_reader_area_images_element(&data));
        let images: Vec<ReaderAreaImage> =
            handle_rawkuma_result!(ReaderAreaImage::from_vec_element(images_elements));
        RawKumaResult::Ok(handle_other_error!(ReaderAreaBuilder::default()
            .images(images)
            .build()))
    }
}

#[derive(Builder, Clone, Serialize)]
pub struct ReaderAreaImage {
    pub index: u32,
    pub server: String,
    pub url: Url,
}

impl<'a> ReaderAreaImage {
    pub fn get_reader_area_image_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse("img.ts-main-image")))
    }
    pub fn get_reader_area_images_element(
        data: &'a ElementRef<'a>,
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
        let index: u32 = match data.value().attr("data-index") {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "data-index not found",
                ))
            }
            Some(d) => {
                handle_other_error!(d.parse::<u32>())
            }
        };
        let server: String = match data.value().attr("data-server") {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "data-server not found",
                ))
            }
            Some(d) => d.to_string(),
        };
        let url: Url = match data.value().attr("src") {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "src not found",
                ))
            }
            Some(d) => {
                handle_other_error!(Url::parse(d))
            }
        };
        RawKumaResult::Ok(handle_other_error!(ReaderAreaImageBuilder::default()
            .index(index)
            .server(server)
            .url(url)
            .build()))
    }
}
