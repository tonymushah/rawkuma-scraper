use derive_builder::Builder;
use reqwest::Url;
use scraper::{ElementRef, Selector};
use serde::Serialize;

use crate::{handle_other_error, handle_selector_error};

use super::{FromElementRef, RawKumaResult};

#[derive(Builder, Clone, Serialize)]
pub struct BsxTitleData {
    pub title: String,
    pub url: Url,
    pub image: Url,
    pub rating: f64,
}

impl FromElementRef<'_> for BsxTitleData {
    fn from_element_ref(data: ElementRef<'_>) -> RawKumaResult<Self> {
        let title = match data
            .select(&handle_selector_error!(Selector::parse(r#"a"#)))
            .next()
        {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Can't find the a element",
                ))
            }
            Some(d) => d,
        };
        let image = match data
            .select(&handle_selector_error!(Selector::parse("img")))
            .next()
        {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Can't find the img element",
                ))
            }
            Some(d) => d,
        };
        let rating = match data
            .select(&handle_selector_error!(Selector::parse(
                r#"div[class="numscore"]"#
            )))
            .next()
        {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    r#"Can't find the div[class="numscore"] element"#,
                ))
            }
            Some(d) => d,
        };
        RawKumaResult::Ok(handle_other_error!(BsxTitleDataBuilder::default()
            .title(
                match title.value().attr("title") {
                    None => {
                        return RawKumaResult::Io(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            r#"Can't find the title attribute"#,
                        ));
                    }
                    Some(d) => d,
                }
                .to_string(),
            )
            .rating(handle_other_error!(match rating.text().next() {
                None => {
                    return RawKumaResult::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        r#"Can't collect the text content"#,
                    ));
                }
                Some(d) => d,
            }
            .parse::<f64>()))
            .image(handle_other_error!(Url::parse(
                format!(
                    "https:{}",
                    match image.value().attr("src") {
                        None => {
                            return RawKumaResult::Io(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                r#"Can't find the src attribute"#,
                            ));
                        }
                        Some(d) => d,
                    }
                )
                .as_str()
            )))
            .url(handle_other_error!(Url::parse(
                format!(
                    "{}",
                    match title.value().attr("href") {
                        None => {
                            return RawKumaResult::Io(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                r#"Can't find the href attribute"#,
                            ));
                        }
                        Some(d) => d,
                    }
                )
                .as_str()
            )))
            .build()))
    }
}
