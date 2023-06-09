mod bixbox;
mod bsx;
pub mod chapter;
mod chapterlist;
mod error;
mod genre_tag;
pub mod home;
pub mod manga;
mod reader_area;
mod utao;
pub mod search;

pub use reader_area::{
    ReaderArea, ReaderAreaBuilder, ReaderAreaBuilderError, ReaderAreaImage, ReaderAreaImageBuilder,
    ReaderAreaImageBuilderError,
};
pub use bixbox::{BixboxData, BixboxDataBuilder, BixboxDataBuilderError};
pub use bsx::{BsxTitleData, BsxTitleDataBuilder, BsxTitleDataBuilderError};
pub use chapterlist::{
    Chapter, ChapterBuilder, ChapterBuilderError, ChapterList, ChapterListBuilder,
    ChapterListBuilderError,
};
pub use error::RawKumaResult;
pub use genre_tag::{MgenTag, MgenTagBuilder, MgenTagBuilderError};
use scraper::{ElementRef};
pub use utao::{
    UtaoTitleChapter, UtaoTitleChapterBuilder, UtaoTitleChapterBuilderError, UtaoTitleData,
    UtaoTitleDataBuilder, UtaoTitleDataBuilderError,
};

use crate::{parser::HtmlParser};

pub trait FromElementRef<'a> {
    fn from_element_ref(data: ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized;
    fn from_vec_element(elements: Vec<ElementRef<'a>>) -> RawKumaResult<Vec<Self>>
    where
        Self: Sized,
    {
        let mut datas: Vec<Self> = Vec::new();
        for element in elements {
            match Self::from_element_ref(element) {
                RawKumaResult::Ok(d) => {
                    datas.push(d);
                }
                _ => {}
            }
        }
        RawKumaResult::Ok(datas)
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

pub trait ToUrlParam<>{
    fn to_url_param(&self) -> Vec<(String, String)>;
}