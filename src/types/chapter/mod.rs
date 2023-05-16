use super::{RawKumaResult, BsxTitleData};
use crate::{handle_other_error, handle_rawkuma_result, parser::chapter::RawKumaChapterParser};
use derive_builder::Builder;
use serde::Serialize;

use super::{FromHtmlParser, ReaderArea};

#[derive(Clone, Builder, Default, Serialize)]
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
