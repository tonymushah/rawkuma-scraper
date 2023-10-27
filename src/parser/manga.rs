use scraper::ElementRef;

use super::{get_content_element, HtmlParser};
use crate::types::{BixboxData, BsxTitleData, ChapterList, FromElementRef, RawKumaResult};

#[derive(Clone)]
pub struct RawKumaMangaDetailParser<'a> {
    content: ElementRef<'a>,
}

impl<'a> HtmlParser<'a> for RawKumaMangaDetailParser<'a> {
    fn init(html: &'a scraper::Html) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let content = get_content_element(html)?;
        RawKumaResult::Ok(Self { content })
    }
}
