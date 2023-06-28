use derive_builder::Builder;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "getset")]
use getset::{Getters, Setters};

use crate::{
    enums::manga::{Status, Genre, Order, Type},
};

mod details;
mod list_paramed;

pub use details::{RawKumaMangaDetailData, RawKumaMangaDetailDataBuilder, RawKumaMangaDetailDataBuilderError};

use super::ToUrlParam;

#[derive(Builder, Clone)]
#[cfg_attr(feature = "getset", derive(Getters, Setters))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaListParameter{
    page : u32,
    status : Status,
    genre : Vec<Genre>,
    order : Order,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    type_ : Type
}

impl Default for MangaListParameter {
    fn default() -> Self {
        Self { page: 1, status: Default::default(), genre: Default::default(), order: Default::default(), type_: Default::default() }
    }
}

impl ToUrlParam for MangaListParameter {
    fn to_url_param(&self) -> Vec<(String, String)> {
        let mut returns : Vec<(String, String)> = Vec::new();
        returns.push(("page".to_string(), self.page.to_string()));
        returns.push(("status".to_string(), self.status.as_str().to_string()));
        for g in &self.genre {
            returns.push(("genre[]".to_string(), g.as_str().to_string()));
        }
        returns.push(("order".to_string(), self.order.as_str().to_string()));
        returns.push(("type".to_string(), self.type_.as_str().to_string()));
        returns
    }
}

