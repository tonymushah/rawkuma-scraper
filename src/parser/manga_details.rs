use scraper::ElementRef;

use super::{get_content_element, HtmlParser};
use crate::handle_rawkuma_result;
use crate::types::{RawKumaResult, BixboxData, FromElementRef};

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

impl<'a> RawKumaMangaDetailParser<'a> {
    pub fn get_bixbox_data(&self) -> RawKumaResult<BixboxData>{
        let bixbox = match self.content.select(&handle_rawkuma_result!(BixboxData::get_bix_box_anime_full_selector())).next() {
            None => return RawKumaResult::Io(std::io::Error::new(std::io::ErrorKind::NotFound, r#"div.bixbox.animefull not found"#)),
            Some(d) => d
        };
        BixboxData::from_element_ref(bixbox)
    }
}
