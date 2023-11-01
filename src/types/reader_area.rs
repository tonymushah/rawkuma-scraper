use derive_builder::Builder;
use htmlize::unescape;
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use super::{error::Error, FromElementRef, RawKumaResult};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Builder, Clone, Default)]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct ReaderArea {
    pub images: Vec<ReaderAreaImage>,
}

impl<'a> ReaderArea {
    pub fn get_reader_area_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse("#readerarea")?)
    }
    pub fn get_reader_area_element(data: &'a ElementRef) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_reader_area_selector()?;
        data.select(&selector)
            .next()
            .ok_or(Error::ElementNotFound("#readerarea".to_string()))
    }
    pub fn get_noscript_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse("noscript")?)
    }
    pub fn get_images_data(data: &'a ElementRef) -> RawKumaResult<Vec<ReaderAreaImage>> {
        let selector = Self::get_noscript_selector()?;
        let data = data
            .select(&selector)
            .next()
            .ok_or(Error::ElementNotFound("noscript".to_string()))?;
        let d = unescape(data.inner_html());
        let data = Html::parse_fragment(d.to_string().as_str());
        let images_elements = ReaderAreaImage::get_reader_area_images_element_from_html(&data)?;
        let images: Vec<ReaderAreaImage> = ReaderAreaImage::from_vec_element(&images_elements)?;
        Ok(images)
    }
}

impl<'a> FromElementRef<'a> for ReaderArea {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(
            ReaderAreaBuilder::default()
                .images(Self::get_images_data(data)?)
                .build()?,
        )
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Builder, Clone, Debug)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct ReaderAreaImage {
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub url: Url,
    pub width: f32,
    pub height: f32,
    pub decoding: String,
    pub alt: String,
}

impl<'a> ReaderAreaImage {
    pub fn get_reader_area_image_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse("img")?)
    }
    #[warn(dead_code)]
    pub fn get_reader_area_images_element(
        data: &'a ElementRef<'a>,
    ) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = Self::get_reader_area_image_selector()?;
        RawKumaResult::Ok(data.select(&selector).collect())
    }
    pub fn get_reader_area_images_element_from_html(
        data: &'a Html,
    ) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let selector = Self::get_reader_area_image_selector()?;
        RawKumaResult::Ok(data.select(&selector).collect())
    }
    pub fn get_width_data(data: &'a ElementRef<'a>) -> RawKumaResult<f32> {
        Ok(data.value().attr("width").map_or(Ok(0.0), |d| d.parse())?)
    }
    pub fn get_height_data(data: &'a ElementRef<'a>) -> RawKumaResult<f32> {
        Ok(data.value().attr("height").map_or(Ok(0.0), |d| d.parse())?)
    }
    pub fn get_decoding_data(data: &'a ElementRef<'a>) -> RawKumaResult<String> {
        Ok(data
            .value()
            .attr("decoding")
            .map_or(Default::default(), |d| d.to_string()))
    }
    pub fn get_alt_data(data: &'a ElementRef<'a>) -> RawKumaResult<String> {
        Ok(data
            .value()
            .attr("alt")
            .map_or(Default::default(), |d| d.to_string()))
    }
    pub fn get_url_data(data: &'a ElementRef<'a>) -> RawKumaResult<Url> {
        Ok(data
            .value()
            .attr("src")
            .ok_or(Error::AttributeNotFound {
                name: "src".to_string(),
                element: "img".to_string(),
            })
            .map(|d| format!("http:{}", d))?
            .as_str()
            .parse::<Url>()?)
    }
}

impl<'a> FromElementRef<'a> for ReaderAreaImage {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(
            ReaderAreaImageBuilder::default()
                .width(Self::get_width_data(data)?)
                .height(Self::get_height_data(data)?)
                .decoding(Self::get_decoding_data(data)?)
                .alt(Self::get_alt_data(data)?)
                .url(Self::get_url_data(data)?)
                .build()?,
        )
    }
}
