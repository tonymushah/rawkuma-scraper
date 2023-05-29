use super::{RawKumaResult, BsxTitleData};
use crate::{handle_other_error, handle_rawkuma_result, parser::chapter::RawKumaChapterParser};
use derive_builder::Builder;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "getset")]
use getset::{Getters};

use super::{FromHtmlParser, ReaderArea};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone, Builder, Default)]
pub struct RawKumaChapterData {
    pub reader_area: ReaderArea,
    pub related_mangas : Vec<BsxTitleData>
}

impl<'a> FromHtmlParser<'a, RawKumaChapterParser<'a>> for RawKumaChapterData {
    fn from(parser: RawKumaChapterParser) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(handle_other_error!(RawKumaChapterDataBuilder::default()
            .reader_area(handle_rawkuma_result!(parser.get_reader_area_data()))
            .related_mangas(handle_rawkuma_result!(parser.get_related_manga()))
            .build()))
    }
}
