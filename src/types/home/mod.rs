use std::collections::HashMap;

use derive_builder::Builder;
use serde::Serialize;

use crate::{parser::home::RawKumaHomeParser, handle_other_error};

use super::{BsxTitleData, UtaoTitleData, FromHtmlParser, RawKumaResult};

#[derive(Clone, Builder, Default, Serialize)]
pub struct RawKumaHomeData {
    pub popular_title : Vec<BsxTitleData>,
    pub recommandation : HashMap<String, Vec<BsxTitleData>>,
    pub latest_update : Vec<UtaoTitleData>
}

impl<'a> FromHtmlParser<'a, RawKumaHomeParser<'a>> for RawKumaHomeData {
    fn from(home : RawKumaHomeParser) -> RawKumaResult<Self> {
        let data = handle_other_error!(RawKumaHomeDataBuilder::default()
            .popular_title(home.get_popular_today())
            .recommandation(home.get_recommandation())
            .latest_update(home.get_latest())
            .build());
        RawKumaResult::Ok(data)
    }
}