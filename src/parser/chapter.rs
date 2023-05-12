use scraper::ElementRef;

use super::{get_content_element, HtmlParser};
use crate::handle_rawkuma_result;
use crate::types::{RawKumaResult, BixboxData, FromElementRef, ChapterList, BsxTitleData, ReaderArea};

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
        RawKumaResult::Ok(Self { content: content })
    }
}

impl<'a> RawKumaChapterParser<'a> {
    pub fn get_reader_area_data(&self) -> RawKumaResult<ReaderArea> {
        todo!()
    }
}