pub mod bixbox;
pub mod bsx;
pub mod chapter;
pub mod chapterlist;
pub mod error;
pub mod genre_tag;
pub mod home;
pub mod manga;
pub mod search;
pub mod utao;

pub use bixbox::{BixboxData, BixboxDataBuilder};
pub use bsx::{BsxTitleData, BsxTitleDataBuilder};
pub use chapterlist::{Chapter, ChapterBuilder, ChapterList, ChapterListBuilder};
pub use error::RawKumaResult;
pub use genre_tag::{MgenTag, MgenTagBuilder};
use scraper::ElementRef;
pub use utao::{UtaoTitleChapter, UtaoTitleChapterBuilder, UtaoTitleData, UtaoTitleDataBuilder};

use crate::parser::HtmlParser;

pub trait FromElementRef<'a> {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized;
    fn from_vec_element(elements: &'a Vec<ElementRef<'a>>) -> RawKumaResult<Vec<Self>>
    where
        Self: Sized,
    {
        let mut datas: Vec<Self> = Vec::new();
        for element in elements {
            if let Ok(d) = Self::from_element_ref(element) {
                datas.push(d);
            }
        }
        Ok(datas)
    }
}

pub trait FromHtmlParser<'a, T>
where
    T: HtmlParser<'a>,
{
    fn from(parser: T) -> RawKumaResult<Self>
    where
        Self: Sized;
}

pub trait ToUrlParam {
    fn to_url_param(&self) -> Vec<(String, String)>;
}
