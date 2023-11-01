use scraper::{ElementRef, Selector};

use super::{get_content_element, HtmlParser};
use crate::types::{error::Error, BsxTitleData, FromElementRef, RawKumaResult, ReaderArea};

#[derive(Clone)]
pub struct RawKumaChapterParser<'a> {
    content: ElementRef<'a>,
}

impl<'a> HtmlParser<'a> for RawKumaChapterParser<'a> {
    fn init(html: &'a scraper::Html) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let content = get_content_element(html)?;
        RawKumaResult::Ok(Self { content })
    }
}

impl<'a> RawKumaChapterParser<'a> {
    pub fn get_reader_area_data(&self) -> RawKumaResult<ReaderArea> {
        let reader_area = ReaderArea::get_reader_area_element(&(self.content))?;
        RawKumaResult::Ok(ReaderArea::from_element_ref(&reader_area)?)
    }
    pub fn get_related_manga(&self) -> RawKumaResult<Vec<BsxTitleData>> {
        let bsx_elements = BsxTitleData::get_bsx_elements(&self.content)?;
        BsxTitleData::from_vec_element(bsx_elements)
    }
    pub fn get_entry_title(&self) -> RawKumaResult<String> {
        let h1_selector = Selector::parse("h1")?;
        match self.content.select(&h1_selector).next() {
            None => RawKumaResult::Err(Error::ElementNotFound("h1".to_string())),
            Some(title) => {
                let titles: Vec<&str> = title.text().collect();
                let title: String = titles.concat();
                RawKumaResult::Ok(title)
            }
        }
    }
}
