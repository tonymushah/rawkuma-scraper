use derive_builder::Builder;

use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::{Getters, Setters};

use crate::parser::manga_details::RawKumaMangaDetailParser;

use crate::types::{
    chapterlist::ChapterList, BixboxData, BsxTitleData, FromHtmlParser, RawKumaResult,
};

#[derive(Serialize, Deserialize, Clone, Builder, Default)]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct RawKumaMangaDetailData {
    pub data: BixboxData,
    pub chapterlist: ChapterList,
    pub related_series: Vec<BsxTitleData>,
}

impl<'a> FromHtmlParser<'a, RawKumaMangaDetailParser<'a>> for RawKumaMangaDetailData {
    fn from(parser: RawKumaMangaDetailParser<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(
            RawKumaMangaDetailDataBuilder::default()
                .data(parser.get_bixbox_data()?)
                .chapterlist(parser.get_chapter_list()?)
                .related_series(parser.get_related_series()?)
                .build()?,
        )
    }
}
