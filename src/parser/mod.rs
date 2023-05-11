use scraper::{Html, Selector, ElementRef};

use crate::{types::RawKumaResult, handle_selector_error, handle_rawkuma_result};

pub mod home;
pub mod manga;
pub mod manga_details;

pub trait HtmlParser<'a> {
    fn init(html: &'a Html) -> RawKumaResult<Self> where Self: Sized;
}

pub fn get_content_selector() -> RawKumaResult<Selector> {
    RawKumaResult::Ok(handle_selector_error!(Selector::parse("div#content")))
}

pub fn get_content_element<'a>(html : &'a Html) -> RawKumaResult<ElementRef<'a>>{
    let selector = handle_rawkuma_result!(get_content_selector());
    match html.select(&selector).next(){
        None => RawKumaResult::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "#content element not found")),
        Some(d) => RawKumaResult::Ok(d)
    }
}