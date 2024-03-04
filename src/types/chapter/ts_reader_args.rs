use scraper::{ElementRef, Selector};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use crate::types::{error::Error, FromElementRef, RawKumaResult};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ImagesSources {
    pub source: String,
    pub images: Vec<Url>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "getset", derive(Getters))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TSReaderArgs {
    pub post_id: usize,
    #[serde(alias = "prevUrl")]
    #[serde(deserialize_with = "deserialize_with_option_url")]
    pub prev_url: Option<Url>,
    #[serde(alias = "nextUrl")]
    #[serde(deserialize_with = "deserialize_with_option_url")]
    pub next_url: Option<Url>,
    pub sources: Vec<ImagesSources>,
}

fn deserialize_with_option_url<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Unexpected, Visitor};

    struct OptionUrlVisitor;

    impl<'de> Visitor<'de> for OptionUrlVisitor {
        type Value = Option<Url>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing an URL")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if s.is_empty() {
                return Ok(None);
            }
            Url::parse(s).map(Some).map_err(|err| {
                let err_s = format!("{}", err);
                Error::invalid_value(Unexpected::Str(s), &err_s.as_str())
            })
        }
    }
    deserializer.deserialize_str(OptionUrlVisitor)
}

fn extract_json_from_string<T: for<'a> Deserialize<'a>>(input: &str) -> RawKumaResult<T> {
    let mut out = String::new();
    let mut brackets = 0;
    let mut is_start = true;
    input.chars().for_each(|c| {
        if c == '{' {
            brackets += 1;
            if is_start {
                is_start = false;
            }
        } else if c == '}' {
            brackets -= 1;
            if !is_start && brackets == 0 {
                out.push(c);
            }
        }
        if brackets != 0 {
            out.push(c);
        }
    });
    Ok(serde_json::from_str(&out)?)
}

impl TSReaderArgs {
    pub fn get_ts_reader_script_selector() -> RawKumaResult<Selector> {
        Ok(Selector::parse("div.readingnav.rnavbot + script")?)
    }
    pub fn get_ts_reader_script_element<'a>(
        element: &'a ElementRef<'a>,
    ) -> RawKumaResult<ElementRef<'a>> {
        let selector = Self::get_ts_reader_script_selector()?;
        element
            .select(&selector)
            .next()
            .ok_or(Error::ElementNotFound(String::from(
                "div.readingnav.rnavbot + script",
            )))
    }
}

impl<'a> FromElementRef<'a> for TSReaderArgs {
    fn from_element_ref(data: &'a ElementRef<'a>) -> RawKumaResult<Self>
    where
        Self: Sized,
    {
        let s: Self = extract_json_from_string(data.inner_html().as_str())?;
        RawKumaResult::Ok(s)
    }
}
