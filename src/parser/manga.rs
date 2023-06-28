use scraper::ElementRef;

use super::{get_content_element, HtmlParser};
use crate::handle_rawkuma_result;
use crate::types::{RawKumaResult, BixboxData, FromElementRef, ChapterList, BsxTitleData};

#[derive(Clone)]
pub struct RawKumaMangaDetailParser<'a> {
    content: ElementRef<'a>,
}

impl<'a> HtmlParser<'a> for RawKumaMangaDetailParser<'a> {
    fn init(html: &'a scraper::Html) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let content = handle_rawkuma_result!(get_content_element(html));
        RawKumaResult::Ok(Self { content: content })
    }
}