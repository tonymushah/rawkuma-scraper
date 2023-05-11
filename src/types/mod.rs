mod bixbox;
mod bsx;
mod error;
mod genre_tag;
pub mod home;
pub mod manga;
mod utao;

pub use bixbox::{BixboxData, BixboxDataBuilder, BixboxDataBuilderError};
pub use bsx::{BsxTitleData, BsxTitleDataBuilder, BsxTitleDataBuilderError};
pub use error::RawKumaResult;
pub use genre_tag::{MgenTag, MgenTagBuilder, MgenTagBuilderError};
use scraper::ElementRef;
pub use utao::{
    UtaoTitleChapter, UtaoTitleChapterBuilder, UtaoTitleChapterBuilderError, UtaoTitleData,
    UtaoTitleDataBuilder, UtaoTitleDataBuilderError,
};

use crate::parser::HtmlParser;

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
        T : HtmlParser<'a>
{
    fn from(parser : T) -> RawKumaResult<Self> where Self: Sized;
}