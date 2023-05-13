use derive_builder::Builder;
use reqwest::Url;
use scraper::{ElementRef, Selector};
use serde::Serialize;

use crate::{handle_other_error, handle_rawkuma_result, handle_selector_error};

use super::{RawKumaResult, FromElementRef};

#[derive(Builder, Clone, Serialize)]
pub struct UtaoTitleChapter {
    pub url: Url,
    pub text: String,
}

impl FromElementRef<'_> for UtaoTitleChapter {
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
        RawKumaResult::Ok(handle_other_error!(UtaoTitleChapterBuilder::default()
            .url(handle_other_error!(Url::parse(
                match title.value().attr("href") {
                    None => {
                        return RawKumaResult::Io(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Can't find href in the a element",
                        ));
                    }
                    Some(d) => d,
                }
            )))
            .text(match title.text().next() {
                None => {
                    return RawKumaResult::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Can't find text in the element",
                    ));
                }
                Some(d) => d.to_string(),
            })
            .build()))
    }
}

#[derive(Builder, Clone, Serialize)]
pub struct UtaoTitleData {
    pub title: String,
    pub url: Url,
    pub image: Url,
    pub chapters: Vec<UtaoTitleChapter>,
}

impl<'a> UtaoTitleData {
    pub fn div_imagu_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(
            r#"div[class="imgu"]"#
        )))
    }

    pub fn get_imgu_div(html: ElementRef<'a>) -> RawKumaResult<ElementRef<'a>> {
        let divs: ElementRef = match html
            .select(&handle_rawkuma_result!(Self::div_imagu_selector()))
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
        return RawKumaResult::Ok(divs);
    }

}

impl FromElementRef<'_> for UtaoTitleData {
    fn from_element_ref(data: ElementRef<'_>) -> RawKumaResult<Self> {
        let imgu = handle_rawkuma_result!(Self::get_imgu_div(data));
        let image = match imgu
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
        let title = match imgu
            .select(&handle_selector_error!(Selector::parse(
                r#"a[class="series"]"#
            )))
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
        let chapters: Vec<ElementRef> = match data
            .select(&handle_selector_error!(Selector::parse(
                r#"ul[class="Manga"]"#
            )))
            .next()
        {
            None => {
                return RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    r#"Can't find the ul[class="Manga"] element"#,
                ))
            }
            Some(d) => d
                .select(&handle_selector_error!(Selector::parse("li")))
                .collect(),
        };
        RawKumaResult::Ok(handle_other_error!(UtaoTitleDataBuilder::default()
            .chapters(handle_rawkuma_result!(
                UtaoTitleChapter::from_vec_element(chapters)
            ))
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
            .build()))
    }

}