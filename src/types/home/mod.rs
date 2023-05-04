use std::collections::HashMap;

use derive_builder::Builder;
use serde::Serialize;

use super::{BsxTitleData, UtaoTitleData};

#[derive(Clone, Builder, Default, Serialize)]
pub struct RawKumaHomeData {
    pub popular_title : Vec<BsxTitleData>,
    pub recommandation : HashMap<String, Vec<BsxTitleData>>,
    pub latest_update : Vec<UtaoTitleData>
}