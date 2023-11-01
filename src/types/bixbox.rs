use derive_builder::Builder;
use reqwest::Url;
use scraper::{ElementRef, Selector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use crate::constant::BASE_URL;

use super::{FromElementRef, MgenTag, RawKumaResult};

use chrono::{DateTime, FixedOffset};

#[derive(Builder, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct BixboxData {
    pub name: String,
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub image: Url,
    pub worst_rating: u16,
    pub best_rating: u16,
    pub rating_count: u32,
    pub rating_value: f32,
    pub description: Option<String>,
    pub author: String,
    pub date_published: DateTime<FixedOffset>,
    pub date_modified: DateTime<FixedOffset>,
    pub title: String,
    pub genres: Vec<MgenTag>,
}

impl Default for BixboxData {
    fn default() -> Self {
        Self {
            name: Default::default(),
            image: Url::parse(BASE_URL).expect("error on parsing the base url"),
            worst_rating: Default::default(),
            best_rating: Default::default(),
            rating_count: Default::default(),
            rating_value: Default::default(),
            description: Default::default(),
            author: Default::default(),
            date_published: Default::default(),
            date_modified: Default::default(),
            title: Default::default(),
            genres: Default::default(),
        }
    }
}

impl BixboxData {
    pub fn get_bix_box_anime_full_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse("div.bixbox.animefull")?)
    }
    pub fn get_image_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"img[itemprop="image"]"#)?)
    }
    pub fn get_name_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"h1[itemprop="name"]"#)?)
    }
    pub fn get_worst_rating_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"meta[itemprop="worstRating"]"#)?)
    }
    pub fn get_best_rating_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"meta[itemprop="bestRating"]"#)?)
    }
    pub fn get_rating_count_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"meta[itemprop="ratingCount"]"#)?)
    }
    pub fn get_rating_value_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[itemprop="ratingValue"]"#)?)
    }
    pub fn get_author_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"span[itemprop="author"]"#)?)
    }
    pub fn get_author_i_selector_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"i[itemprop="name"]"#)?)
    }
    pub fn get_description_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[itemprop="description"]"#)?)
    }
    pub fn get_date_modified_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"time[itemprop="dateModified"]"#)?)
    }
    pub fn get_date_published_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"time[itemprop="datePublished"]"#)?)
    }

    pub fn get_image_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_image_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"img[itemprop="image"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_name_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_name_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"h1[itemprop="name"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_worst_rating_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_worst_rating_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"meta[itemprop="worstRating"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_best_rating_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_best_rating_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"meta[itemprop="bestRating"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_rating_count_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_rating_count_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"meta[itemprop="ratingCount"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_rating_value_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_rating_value_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"div[itemprop="ratingValue"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_author_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_author_selector()?;
        let i_selector = Self::get_author_i_selector_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"span[itemprop="author"]"#.to_string(),
            )),
            Some(d) => match d.select(&i_selector).next() {
                None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                    r#"i[itemprop="name"]"#.to_string(),
                )),
                Some(d) => RawKumaResult::Ok(d),
            },
        }
    }
    pub fn get_description_element<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_description_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"div[itemprop="description"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_date_modified_element<'a>(
        data: &'a ElementRef<'a>,
    ) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_date_modified_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"time[itemprop="dateModified"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }
    pub fn get_date_published_element<'a>(
        data: &'a ElementRef<'a>,
    ) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_date_published_selector()?;
        match data.select(&selector).next() {
            None => RawKumaResult::Err(super::error::Error::ElementNotFound(
                r#"time[itemprop="datePublished"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }

    pub fn get_image_element_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<(Url, String)> {
        let element = Self::get_image_element(data)?;
        let url = Url::parse(
            format!(
                "https:{}",
                match element.value().attr("src") {
                    None => {
                        return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                            name: "src".to_string(),
                            element: r#"img[itemprop="image"]"#.to_string(),
                        });
                    }
                    Some(d) => d,
                }
            )
            .as_str(),
        )?;
        let title: String = match element.value().attr("title") {
            None => {
                return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                    name: "title".to_string(),
                    element: r#"img[itemprop="image"]"#.to_string(),
                });
            }
            Some(d) => d.to_string(),
        };
        RawKumaResult::Ok((url, title))
    }
    pub fn get_name_element_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<String> {
        let element = Self::get_name_element(data)?;
        match element.text().next() {
            None => RawKumaResult::Err(super::error::Error::TextContentFound),
            Some(d) => RawKumaResult::Ok(d.to_string()),
        }
    }
    pub fn get_worst_rating_element_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<u16> {
        let element = Self::get_worst_rating_element(data)?;
        let content = match element.value().attr("content") {
            None => {
                return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                    name: "content".to_string(),
                    element: r#"meta[itemprop="worstRating"]"#.to_string(),
                })
            }
            Some(d) => d.parse::<u16>()?,
        };
        RawKumaResult::Ok(content)
    }
    pub fn get_best_rating_element_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<u16> {
        let element = Self::get_best_rating_element(data)?;
        let content = match element.value().attr("content") {
            None => {
                return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                    name: "content".to_string(),
                    element: r#"meta[itemprop="bestRating"]"#.to_string(),
                })
            }
            Some(d) => d.parse::<u16>()?,
        };
        RawKumaResult::Ok(content)
    }
    pub fn get_rating_count_element_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<u32> {
        let element = Self::get_rating_count_element(data)?;
        let content = match element.value().attr("content") {
            None => {
                return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                    name: "content".to_string(),
                    element: r#"meta[itemprop="ratingCount"]"#.to_string(),
                })
            }
            Some(d) => d.parse::<u32>()?,
        };
        RawKumaResult::Ok(content)
    }
    pub fn get_rating_value_element_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<f32> {
        let element = Self::get_rating_value_element(data)?;
        let content = match element.value().attr("content") {
            None => {
                return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                    name: "content".to_string(),
                    element: r#"div[itemprop="ratingValue"]"#.to_string(),
                })
            }
            Some(d) => d.parse::<f32>()?,
        };
        RawKumaResult::Ok(content)
    }
    pub fn get_author_element_data<'a>(data: &'a ElementRef<'a>) -> RawKumaResult<String> {
        let element = Self::get_author_element(data)?;
        match element.text().next() {
            None => RawKumaResult::Err(super::error::Error::TextContentFound),
            Some(d) => RawKumaResult::Ok(d.to_string()),
        }
    }
    pub fn get_description_element_data<'a>(
        data: &'a ElementRef<'a>,
    ) -> RawKumaResult<Option<String>> {
        let element = Self::get_description_element(data)?;
        match element.text().next() {
            None => RawKumaResult::Ok(None),
            Some(d) => RawKumaResult::Ok(Some(d.to_string())),
        }
    }
    pub fn get_date_modified_element_data<'a>(
        data: &'a ElementRef<'a>,
    ) -> RawKumaResult<DateTime<FixedOffset>> {
        let element = Self::get_date_modified_element(data)?;
        let datetime = match element.value().attr("datetime") {
            None => {
                return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                    name: "datetime".to_string(),
                    element: r#"time[itemprop="dateModified"]"#.to_string(),
                })
            }
            Some(d) => DateTime::parse_from_rfc3339(d)?,
        };
        RawKumaResult::Ok(datetime)
    }
    pub fn get_date_published_element_data<'a>(
        data: &'a ElementRef<'a>,
    ) -> RawKumaResult<DateTime<FixedOffset>> {
        let element = Self::get_date_published_element(data)?;
        let datetime = match element.value().attr("datetime") {
            None => {
                return RawKumaResult::Err(super::error::Error::AttributeNotFound {
                    name: "datetime".to_string(),
                    element: r#"time[itemprop="datePublished"]"#.to_string(),
                })
            }
            Some(d) => DateTime::parse_from_rfc3339(d)?,
        };
        RawKumaResult::Ok(datetime)
    }
}

impl<'a> FromElementRef<'a> for BixboxData {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let (image, title) = Self::get_image_element_data(data)?;
        RawKumaResult::Ok(
            BixboxDataBuilder::default()
                .image(image)
                .title(title)
                .name(Self::get_name_element_data(data)?)
                .worst_rating(Self::get_worst_rating_element_data(data)?)
                .best_rating(Self::get_best_rating_element_data(data)?)
                .rating_count(Self::get_rating_count_element_data(data)?)
                .rating_value(Self::get_rating_value_element_data(data)?)
                .description(Self::get_description_element_data(data)?)
                .author(Self::get_author_element_data(data)?)
                .date_modified(Self::get_date_modified_element_data(data)?)
                .date_published(Self::get_date_published_element_data(data)?)
                .genres(MgenTag::get_tags_elements_data(data)?)
                .build()?,
        )
    }
}
