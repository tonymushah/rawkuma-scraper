use derive_builder::Builder;
use serde::Serialize;

use crate::{handle_other_error, handle_rawkuma_result, parser::search::RawKumaSearchParser};

use super::{BsxTitleData, FromHtmlParser, RawKumaResult};

#[derive(Default, Serialize, Builder)]
pub struct RawKumaSearch {
    pub result: Vec<BsxTitleData>,
}

impl<'a> FromHtmlParser<'a, RawKumaSearchParser<'a>> for RawKumaSearch {
    fn from(parser: RawKumaSearchParser<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(handle_other_error!(RawKumaSearchBuilder::default()
            .result(handle_rawkuma_result!(parser.get_bsx_results()))
            .build()))
    }
}
