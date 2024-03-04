use derive_builder::Builder;

use serde::{Deserialize, Serialize};

#[cfg(feature = "getset")]
use getset::{Getters, Setters};

use crate::enums::manga::{Genre, Order, Status, Type};

mod details;

pub use details::{RawKumaMangaDetailData, RawKumaMangaDetailDataBuilder};

use super::ToUrlParam;

#[derive(Builder, Clone)]
#[cfg_attr(feature = "getset", derive(Getters, Setters))]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[builder(build_fn(error = "crate::types::error::BuilderError"))]
pub struct MangaListParameter {
    page: u32,
    status: Status,
    genre: Vec<Genre>,
    order: Order,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    type_: Type,
}

impl Default for MangaListParameter {
    fn default() -> Self {
        Self {
            page: 1,
            status: Default::default(),
            genre: Default::default(),
            order: Default::default(),
            type_: Default::default(),
        }
    }
}

impl ToUrlParam for MangaListParameter {
    fn to_url_param(&self) -> Vec<(String, String)> {
        let mut returns: Vec<(String, String)> = Vec::new();
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
