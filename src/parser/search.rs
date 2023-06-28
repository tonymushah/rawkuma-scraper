use scraper::ElementRef;

use super::{get_content_element, HtmlParser};
use crate::handle_rawkuma_result;
use crate::types::{RawKumaResult, FromElementRef, BsxTitleData};

#[derive(Clone)]
pub struct RawKumaSearchParser<'a> {
    content: ElementRef<'a>,
}

impl<'a> HtmlParser<'a> for RawKumaSearchParser<'a> {
    fn init(html: &'a scraper::Html) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let content = handle_rawkuma_result!(get_content_element(html));
        RawKumaResult::Ok(Self { content: content })
    }
}

impl<'a> RawKumaSearchParser<'a> {
    pub fn get_bsx_results(&'a self) -> RawKumaResult<Vec<BsxTitleData>> {
        let elements : Vec<ElementRef<'a>> = handle_rawkuma_result!(BsxTitleData::get_bsx_elements(&self.content));
        RawKumaResult::Ok(handle_rawkuma_result!(BsxTitleData::from_vec_element(elements)))
    }
}