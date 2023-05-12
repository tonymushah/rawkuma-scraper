use derive_builder::Builder;
use serde::Serialize;

use crate::{
    handle_other_error, handle_rawkuma_result, parser::manga_details::RawKumaMangaDetailParser,
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
