use super::RawKumaResult;
use crate::{handle_other_error, handle_rawkuma_result, parser::chapter::RawKumaChapterParser};
use derive_builder::Builder;
use serde::Serialize;

use super::{FromHtmlParser, ReaderArea};

#[derive(Clone, Builder, Default, Serialize)]
pub struct RawKumaChapterData {
    pub reader_area: ReaderArea,
}

impl<'a> FromHtmlParser<'a, RawKumaChapterParser<'a>> for RawKumaChapterData {
    fn from(parser: RawKumaChapterParser) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(handle_other_error!(RawKumaChapterDataBuilder::default()
            .reader_area(handle_rawkuma_result!(parser.get_reader_area_data()))
            .build()))
    }
}
