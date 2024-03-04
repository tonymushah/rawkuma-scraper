pub mod ts_reader_args;

use self::ts_reader_args::TSReaderArgs;

use super::{BsxTitleData, RawKumaResult};
use crate::parser::chapter::RawKumaChapterParser;
use derive_builder::Builder;

use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use super::FromHtmlParser;

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone, Builder)]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct RawKumaChapterData {
    pub title: String,
    pub sources: TSReaderArgs,
    pub related_mangas: Vec<BsxTitleData>,
}

impl<'a> FromHtmlParser<'a, RawKumaChapterParser<'a>> for RawKumaChapterData {
    fn from(parser: RawKumaChapterParser) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(
            RawKumaChapterDataBuilder::default()
                .title(parser.get_entry_title()?)
                .sources(parser.get_ts_reader_args()?)
                .related_mangas(parser.get_related_manga()?)
                .build()?,
        )
    }
}
