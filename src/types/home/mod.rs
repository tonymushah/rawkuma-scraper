use std::collections::HashMap;

use derive_builder::Builder;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "getset")]
use getset::{Getters};

use crate::{parser::home::RawKumaHomeParser, handle_other_error};

use super::{BsxTitleData, UtaoTitleData, FromHtmlParser, RawKumaResult, ToUrlParam};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone, Builder, Default)]
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

#[derive(Clone, Builder)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct HomeParameters{
    pub page : u32
}

impl Default for HomeParameters {
    fn default() -> Self {
        Self { page: 1 }
    }
}

impl ToUrlParam for HomeParameters{
    fn to_url_param(&self) -> Vec<(String, String)> {
        let mut vec : Vec<(String, String)> = Vec::new();
        vec.push(("page".to_string(), format!("{}", self.page)));
        return vec;
    }
}