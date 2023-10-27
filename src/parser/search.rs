use scraper::ElementRef;

use super::{get_content_element, HtmlParser};
use crate::types::{BsxTitleData, FromElementRef, RawKumaResult};

#[derive(Clone)]
pub struct RawKumaSearchParser<'a> {
    content: ElementRef<'a>,
}

impl<'a> HtmlParser<'a> for RawKumaSearchParser<'a> {
    fn init(html: &'a scraper::Html) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let content = get_content_element(html)?;
        RawKumaResult::Ok(Self { content })
    }
}

impl<'a> RawKumaSearchParser<'a> {
    pub fn get_bsx_results(&'a self) -> RawKumaResult<Vec<BsxTitleData>> {
        let elements: Vec<ElementRef<'a>> = BsxTitleData::get_bsx_elements(&self.content)?;
        RawKumaResult::Ok(BsxTitleData::from_vec_element(elements)?)
    }
}
