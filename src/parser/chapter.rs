use scraper::{ElementRef, Selector};

use super::{get_content_element, HtmlParser};
use crate::{handle_rawkuma_result, handle_selector_error};
use crate::types::{RawKumaResult, FromElementRef, BsxTitleData, ReaderArea};

#[derive(Clone)]
pub struct RawKumaChapterParser<'a> {
    content: ElementRef<'a>,
}

impl<'a> HtmlParser<'a> for RawKumaChapterParser<'a> {
    fn init(html: &'a scraper::Html) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let content = handle_rawkuma_result!(get_content_element(html));
        RawKumaResult::Ok(Self { content })
    }
}

impl<'a> RawKumaChapterParser<'a> {
    pub fn get_reader_area_data(&self) -> RawKumaResult<ReaderArea> {
        let reader_area = handle_rawkuma_result!(ReaderArea::get_reader_area_element(&(self.content)));
        RawKumaResult::Ok(handle_rawkuma_result!(ReaderArea::from_element_ref(reader_area)))
    }
    pub fn get_related_manga(&self) -> RawKumaResult<Vec<BsxTitleData>> {
        let bsx_elements = handle_rawkuma_result!(BsxTitleData::get_bsx_elements(&self.content));
        BsxTitleData::from_vec_element(bsx_elements)
    }
    pub fn get_entry_title(&self) -> RawKumaResult<String> {
        let h1_selector = handle_selector_error!(Selector::parse("h1."));
        match self.content.select(&h1_selector).next() {
            None => Ok()
        }
    }
}