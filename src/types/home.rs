use std::collections::HashMap;

use derive_builder::Builder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::Getters;

use crate::parser::home::RawKumaHomeParser;

use super::{BsxTitleData, FromHtmlParser, RawKumaResult, UtaoTitleData};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone, Builder, Default)]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct RawKumaHomeData {
    pub popular_title: Vec<BsxTitleData>,
    pub recommandation: HashMap<String, Vec<BsxTitleData>>,
    pub latest_update: Vec<UtaoTitleData>,
}

impl<'a> FromHtmlParser<'a, RawKumaHomeParser<'a>> for RawKumaHomeData {
    fn from(home: RawKumaHomeParser) -> RawKumaResult<Self> {
        let data = RawKumaHomeDataBuilder::default()
            .popular_title(home.get_popular_today())
            .recommandation(home.get_recommandation())
            .latest_update(home.get_latest())
            .build()?;
        RawKumaResult::Ok(data)
    }
}
