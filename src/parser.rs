use scraper::{ElementRef, Html, Selector};

use crate::types::RawKumaResult;

pub mod chapter;
pub mod home;
pub mod manga;
pub mod manga_details;
pub mod search;

pub trait HtmlParser<'a> {
    fn init(html: &'a Html) -> RawKumaResult<Self>
    where
        Self: Sized;
}

pub fn get_content_selector() -> RawKumaResult<Selector> {
    RawKumaResult::Ok(Selector::parse("div#content")?)
}

pub fn get_content_element(html: &Html) -> RawKumaResult<ElementRef<'_>> {
    let selector = get_content_selector()?;
    match html.select(&selector).next() {
        None => RawKumaResult::Err(crate::types::error::Error::ElementNotFound(String::from(
            "#content",
        ))),
        Some(d) => RawKumaResult::Ok(d),
    }
}
