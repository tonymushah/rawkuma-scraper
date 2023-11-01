use std::collections::HashMap;

use derive_builder::Builder;
use scraper::{ElementRef, Html, Selector};

use crate::types::{error::Error, BsxTitleData, FromElementRef, RawKumaResult, UtaoTitleData};

use super::HtmlParser;

#[derive(Clone, Builder)]
pub struct RawKumaHomeParser<'a> {
    #[builder(setter(skip = true))]
    popular_today: Vec<ElementRef<'a>>,
    #[builder(setter(skip = true))]
    recommandation: HashMap<String, Vec<ElementRef<'a>>>,
    #[builder(setter(skip = true))]
    utao_elements: Vec<ElementRef<'a>>,
}

impl<'a> HtmlParser<'a> for RawKumaHomeParser<'a> {
    fn init(html: &'a Html) -> RawKumaResult<Self> {
        let popular_today = Self::find_popular_today_elements(html)?;
        let recommandation = Self::find_recomendation_elements(html)?;
        let utaos = Self::get_utao_divs(html)?;
        RawKumaResult::Ok(Self {
            popular_today,
            recommandation,
            utao_elements: utaos,
        })
    }
}

impl<'a> RawKumaHomeParser<'a> {
    pub fn div_listupd_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[class="listupd"]"#)?)
    }

    pub fn get_div_listupd(html: &Html) -> RawKumaResult<Vec<ElementRef>> {
        let divs: Vec<ElementRef> = html.select(&(Self::div_listupd_selector()?)).collect();
        RawKumaResult::Ok(divs)
    }

    pub fn div_bixbox_hothome_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[class="bixbox hothome"]"#)?)
    }

    pub fn get_div_bixbox_hothome(html: &'a Html) -> RawKumaResult<ElementRef<'a>> {
        let op: Vec<ElementRef> = html.select(&(Self::div_listupd_selector()?)).collect();
        match op.get(0) {
            None => RawKumaResult::Err(crate::types::error::Error::ElementNotFound(
                "div[class='bixbox hothome']".to_string(),
            )),
            Some(d) => RawKumaResult::Ok(*d),
        }
    }

    pub fn find_popular_today_elements(html: &'a Html) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let div = Self::get_div_bixbox_hothome(html)?;
        let bsx_elements: Vec<ElementRef> =
            div.select(&(BsxTitleData::div_bsx_selector()?)).collect();
        RawKumaResult::Ok(bsx_elements)
    }

    pub fn div_series_gen_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[class="series-gen"]"#)?)
    }

    pub fn get_div_series_gen(html: &'a Html) -> RawKumaResult<ElementRef<'a>> {
        match html.select(&(Self::div_series_gen_selector()?)).next() {
            None => RawKumaResult::Err(crate::types::error::Error::ElementNotFound(
                r#"div[class="series-gen"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }

    pub fn find_recomendation_elements(
        html: &'a Html,
    ) -> RawKumaResult<HashMap<String, Vec<ElementRef<'a>>>> {
        let recomm = Self::get_recommendation_themes(html)?;
        let mut data: HashMap<String, Vec<ElementRef<'a>>> = HashMap::new();
        for (key, elememt) in recomm {
            data.insert(
                key,
                elememt
                    .select(&(BsxTitleData::div_bsx_selector()?))
                    .collect(),
            );
        }
        RawKumaResult::Ok(data)
    }

    pub fn div_nav_tabs_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"ul[class="nav-tabs"]"#)?)
    }

    pub fn get_div_nav_tabs(html: &'a ElementRef) -> RawKumaResult<ElementRef<'a>> {
        match html.select(&(Self::div_series_gen_selector()?)).next() {
            None => RawKumaResult::Err(crate::types::error::Error::ElementNotFound(
                r#"ul[class="nav-tabs"]"#.to_string(),
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }

    pub fn get_nav_serie_gen_divs(html: &'a Html) -> RawKumaResult<ElementRef<'a>> {
        match html.select(&(Self::div_series_gen_selector()?)).next() {
            None => RawKumaResult::Err(crate::types::error::Error::ElementNotFound(
                r#"div[class="series-gen"]"#.to_string(),
            )),
            Some(d) => match d.select(&(Self::div_nav_tabs_selector()?)).next() {
                None => RawKumaResult::Err(crate::types::error::Error::ElementNotFound(
                    r#"ul[class="nav-tabs"]"#.to_string(),
                )),
                Some(d) => RawKumaResult::Ok(d),
            },
        }
    }

    pub fn get_recommendation_themes(
        html: &'a Html,
    ) -> RawKumaResult<HashMap<String, ElementRef<'a>>> {
        let mut refs: HashMap<String, ElementRef<'a>> = HashMap::new();
        let a = "a".to_string();
        let selector = Selector::parse("a")?;
        for theme in (Self::get_nav_serie_gen_divs(html)?).select(&selector) {
            let text = theme
                .text()
                .next()
                .map(|d| d.to_string())
                .ok_or(crate::types::error::Error::ElementNotFound(a.clone()))?;
            let href = theme
                .value()
                .attr("href")
                .ok_or(crate::types::error::Error::AttributeNotFound {
                    name: "href".to_string(),
                    element: a.clone(),
                })?
                .to_string();
            let selector_ = std::convert::TryInto::<Selector>::try_into(href.as_str())
                .map_err(|e| Error::SelectorErrorKind(e.to_string()))?;
            let element = match html.select(&selector_).next() {
                None => {
                    return RawKumaResult::Err(crate::types::error::Error::ElementNotFound(
                        href.to_string(),
                    ))
                }
                Some(d) => d,
            };
            refs.insert(text, element);
        }
        RawKumaResult::Ok(refs)
    }

    pub fn div_utao_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(Selector::parse(r#"div[class="utao"]"#)?)
    }

    pub fn get_utao_divs(html: &'a Html) -> RawKumaResult<Vec<ElementRef<'a>>> {
        RawKumaResult::Ok(
            html.select(&(Self::div_utao_selector()?))
                .collect::<Vec<ElementRef<'a>>>(),
        )
    }

    pub fn get_popular_today(&self) -> Vec<BsxTitleData> {
        let mut result: Vec<BsxTitleData> = Vec::new();
        for element in &self.popular_today {
            /*


            match BsxTitleData::from_element_ref(*element) {
                RawKumaResult::Ok(d) => {
                    result.push(d);
                }
                _ => {}
            }
            */
            result.push(BsxTitleData::from_element_ref(element).unwrap())
        }
        result
    }

    pub fn get_recommandation(&self) -> HashMap<String, Vec<BsxTitleData>> {
        let mut result: HashMap<String, Vec<BsxTitleData>> = HashMap::new();
        for (title, elements) in &self.recommandation {
            if let RawKumaResult::Ok(d) = BsxTitleData::from_vec_element(elements) {
                result.insert(title.clone(), d);
            }
        }
        result
    }

    pub fn get_latest(&self) -> Vec<UtaoTitleData> {
        let mut result: Vec<UtaoTitleData> = Vec::new();
        for element in &self.utao_elements {
            if let RawKumaResult::Ok(d) = UtaoTitleData::from_element_ref(element) {
                result.push(d);
            }
        }
        result
    }
}
