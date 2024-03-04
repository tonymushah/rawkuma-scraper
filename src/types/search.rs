use derive_builder::Builder;

use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use crate::parser::search::RawKumaSearchParser;

use super::{BsxTitleData, FromHtmlParser, RawKumaResult};

#[derive(Default, Clone, Builder, Serialize, Deserialize)]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct RawKumaSearch {
    pub result: Vec<BsxTitleData>,
}

impl<'a> FromHtmlParser<'a, RawKumaSearchParser<'a>> for RawKumaSearch {
    fn from(parser: RawKumaSearchParser<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        RawKumaResult::Ok(
            RawKumaSearchBuilder::default()
                .result(parser.get_bsx_results()?)
                .build()?,
        )
    }
}
