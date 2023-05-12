use std::collections::HashMap;

use derive_builder::Builder;
use scraper::{ElementRef, Html, Selector};

use crate::{
    handle_rawkuma_result, handle_selector_error,
    types::{BsxTitleData, RawKumaResult, UtaoTitleData, FromElementRef},
};

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
        let popular_today = handle_rawkuma_result!(Self::find_popular_today_elements(&html));
        let recommandation = handle_rawkuma_result!(Self::find_recomendation_elements(&html));
        let utaos = handle_rawkuma_result!(Self::get_utao_divs(&html));
        RawKumaResult::Ok(Self {
            popular_today: popular_today,
            recommandation: recommandation,
            utao_elements: utaos,
        })
    }
}

impl<'a> RawKumaHomeParser<'a> {
    pub fn div_listupd_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(
            r#"div[class="listupd"]"#
        )))
    }

    pub fn get_div_listupd(html: &Html) -> RawKumaResult<Vec<ElementRef>> {
        let divs: Vec<ElementRef> = html
            .select(&handle_rawkuma_result!(Self::div_listupd_selector()))
            .collect();
        return RawKumaResult::Ok(divs);
    }

    pub fn div_bixbox_hothome_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(
            r#"div[class="bixbox hothome"]"#
        )))
    }

    pub fn get_div_bixbox_hothome(html: &'a Html) -> RawKumaResult<ElementRef<'a>> {
        let op: Vec<ElementRef> = html
            .select(&handle_rawkuma_result!(Self::div_listupd_selector()))
            .collect();
        match op.get(0) {
            None => RawKumaResult::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                r#"can't find the div[class="bixbox hothome"] componnent"#,
            )),
            Some(d) => RawKumaResult::Ok(d.clone()),
        }
    }

    pub fn find_popular_today_elements(html: &'a Html) -> RawKumaResult<Vec<ElementRef<'a>>> {
        let div = handle_rawkuma_result!(Self::get_div_bixbox_hothome(&html));
        let bsx_elements: Vec<ElementRef> = div
            .select(&handle_rawkuma_result!(BsxTitleData::div_bsx_selector()))
            .collect();
        RawKumaResult::Ok(bsx_elements)
    }

    pub fn div_series_gen_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(
            r#"div[class="series-gen"]"#
        )))
    }

    pub fn get_div_series_gen(html: &'a Html) -> RawKumaResult<ElementRef<'a>> {
        match html
            .select(&handle_rawkuma_result!(Self::div_series_gen_selector()))
            .next()
        {
            None => RawKumaResult::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                r#"can't find the div[class="series-gen"] componnent"#,
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }

    pub fn find_recomendation_elements(
        html: &'a Html,
    ) -> RawKumaResult<HashMap<String, Vec<ElementRef<'a>>>> {
        let recomm = handle_rawkuma_result!(Self::get_recommendation_themes(html));
        let mut data: HashMap<String, Vec<ElementRef<'a>>> = HashMap::new();
        for (key, elememt) in recomm {
            data.insert(
                key,
                elememt
                    .select(&handle_rawkuma_result!(BsxTitleData::div_bsx_selector()))
                    .collect(),
            );
        }
        RawKumaResult::Ok(data)
    }

    pub fn div_nav_tabs_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(
            r#"ul[class="nav-tabs"]"#
        )))
    }

    pub fn get_div_nav_tabs(html: &'a ElementRef) -> RawKumaResult<ElementRef<'a>> {
        match html
            .select(&handle_rawkuma_result!(Self::div_series_gen_selector()))
            .next()
        {
            None => RawKumaResult::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                r#"can't find the ul[class="nav-tabs"] componnent"#,
            )),
            Some(d) => RawKumaResult::Ok(d),
        }
    }

    pub fn get_nav_serie_gen_divs(html: &'a Html) -> RawKumaResult<ElementRef<'a>> {
        match html
            .select(&handle_rawkuma_result!(Self::div_series_gen_selector()))
            .next()
        {
            None => RawKumaResult::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                r#"can't find the div[class="series-gen"] componnent"#,
            )),
            Some(d) => match d
                .select(&handle_rawkuma_result!(Self::div_nav_tabs_selector()))
                .next()
            {
                None => RawKumaResult::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    r#"can't find the ul[class="nav-tabs"] componnent"#,
                )),
                Some(d) => RawKumaResult::Ok(d),
            },
        }
    }

    pub fn get_recommendation_themes(
        html: &'a Html,
    ) -> RawKumaResult<HashMap<String, ElementRef<'a>>> {
        let mut refs: HashMap<String, ElementRef<'a>> = HashMap::new();
        let selector = handle_selector_error!(Selector::parse("a"));
        for theme in handle_rawkuma_result!(Self::get_nav_serie_gen_divs(html)).select(&selector) {
            let text = match theme.text().next() {
                None => {
                    return RawKumaResult::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        r#"can't find the text in componnent"#,
                    ))
                }
                Some(d) => d.to_string(),
            };
            let href = match theme.value().attr("href") {
                None => {
                    return RawKumaResult::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        r#"can't find the href attribute in componnent"#,
                    ))
                }
                Some(d) => d,
            };
            let selector_ = handle_selector_error!(Selector::parse(href));
            let element = match html.select(&selector_).next() {
                None => {
                    return RawKumaResult::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("can't find the {} componnent", href),
                    ))
                }
                Some(d) => d,
            };
            refs.insert(text, element);
        }
        RawKumaResult::Ok(refs)
    }

    pub fn div_utao_selector() -> RawKumaResult<Selector> {
        RawKumaResult::Ok(handle_selector_error!(Selector::parse(
            r#"div[class="utao"]"#
        )))
    }

    pub fn get_utao_divs(html: &'a Html) -> RawKumaResult<Vec<ElementRef<'a>>> {
        RawKumaResult::Ok(
            html.select(&handle_rawkuma_result!(Self::div_utao_selector()))
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
            result.push(BsxTitleData::from_element_ref(*element).unwrap())
        }
        result
    }

    pub fn get_recommandation(&self) -> HashMap<String, Vec<BsxTitleData>> {
        let mut result: HashMap<String, Vec<BsxTitleData>> = HashMap::new();
        for (title, elements) in &(&self).recommandation {
            match BsxTitleData::from_vec_element(elements.clone()) {
                RawKumaResult::Ok(d) => {
                    result.insert(title.clone(), d);
                }
                _ => {}
            }
        }
        result
    }

    pub fn get_latest(&self) -> Vec<UtaoTitleData> {
        let mut result: Vec<UtaoTitleData> = Vec::new();
        for element in &(&self).utao_elements {
            match UtaoTitleData::from_element_ref(element.clone()) {
                RawKumaResult::Ok(d) => {
                    result.push(d);
                }
                _ => {}
            }
        }
        result
    }
}
