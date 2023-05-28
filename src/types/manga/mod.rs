use derive_builder::Builder;
use serde::{Serialize, Deserialize};

use crate::{
    handle_other_error, handle_rawkuma_result, parser::manga_details::RawKumaMangaDetailParser, enums::manga::{Status, Genre, Order, Type},
};

use super::{BixboxData, FromHtmlParser, RawKumaResult, chapterlist::ChapterList, BsxTitleData};

#[derive(Clone, Builder, Default, Serialize)]
pub struct RawKumaMangaDetailData {
    pub data: BixboxData,
    pub chapterlist : ChapterList,
    pub related_series : Vec<BsxTitleData>
}

impl<'a> FromHtmlParser<'a, RawKumaMangaDetailParser<'a>> for RawKumaMangaDetailData {
    fn from(parser: RawKumaMangaDetailParser<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(handle_other_error!(RawKumaMangaDetailDataBuilder::default(
        )
        .data(handle_rawkuma_result!(parser.get_bixbox_data()))
        .chapterlist(handle_rawkuma_result!(parser.get_chapter_list()))
        .related_series(handle_rawkuma_result!(parser.get_related_series()))
        .build()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct MangaListParameter{
    page : u64,
    status : Status,
    genre : Vec<Genre>,
    order : Order,
    type_ : Type
}

impl<'a> Default for MangaListParameter {
    fn default() -> Self {
        Self { page: 1, status: Default::default(), genre: Default::default(), order: Default::default(), type_: Default::default() }
    }
}