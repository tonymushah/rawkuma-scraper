use derive_builder::Builder;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "getset")]
use getset::{Getters, Setters};

use crate::{
    handle_other_error, handle_rawkuma_result, parser::manga_details::RawKumaMangaDetailParser
};

use crate::types::{BixboxData, FromHtmlParser, RawKumaResult, chapterlist::ChapterList, BsxTitleData};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Builder, Default)]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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
