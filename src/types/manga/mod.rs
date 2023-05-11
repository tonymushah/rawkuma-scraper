use derive_builder::Builder;
use serde::Serialize;

use crate::parser::manga_details::RawKumaMangaDetailParser;

use super::{BixboxData, FromHtmlParser, RawKumaResult};

#[derive(Clone, Builder, Default, Serialize)]
pub struct RawKumaMangaDetailData {
    pub data : BixboxData
}

impl<'a> FromHtmlParser<'a, RawKumaMangaDetailParser<'a>> for RawKumaMangaDetailData {
    fn from(parser : RawKumaMangaDetailParser<'a>) -> RawKumaResult<Self> where Self: Sized {
        todo!()
    }
}