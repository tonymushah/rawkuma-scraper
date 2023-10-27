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

impl<'a> RawKumaMangaDetailParser<'a> {
    pub fn get_bixbox_data(&self) -> RawKumaResult<BixboxData> {
        let bixbox = match self
            .content
            .clone()
            .select(&(BixboxData::get_bix_box_anime_full_selector()?))
            .next()
        {
            None => {
                return RawKumaResult::Err(crate::types::error::Error::ElementNotFound(
                    "div.bixbox.animefull".to_string(),
                ))
            }
            Some(d) => d,
        };
        BixboxData::from_element_ref(bixbox)
    }
    pub fn get_chapter_list(&self) -> RawKumaResult<ChapterList> {
        let chapter_list = ChapterList::get_chapter_list_element(&self.content)?;
        ChapterList::from_element_ref(chapter_list)
    }
    pub fn get_related_series(&self) -> RawKumaResult<Vec<BsxTitleData>> {
        let bsx_elements: Vec<ElementRef<'a>> = self
            .content
            .select(&(BsxTitleData::div_bsx_selector()?))
            .collect();
        BsxTitleData::from_vec_element(bsx_elements)
    }
}
