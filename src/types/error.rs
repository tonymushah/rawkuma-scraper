use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

use derive_builder::UninitializedFieldError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Element '{0}' not found on the page")]
    ElementNotFound(String),
    #[error("Element {} not found on {}", element, parent)]
    ElementNotFoundInNested { element: String, parent: String },
    #[error("Attribute '{}' not found on the element {}", name, element)]
    AttributeNotFound { name: String, element: String },
    #[error(transparent)]
    ScraperParseError(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error(transparent)]
    BuilderError(#[from] BuilderError),
    #[error("Text content is not found")]
    TextContentFound,
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),
}

#[doc = "Error type for Rawkuma Error"]
#[derive(Debug)]
#[non_exhaustive]
pub enum BuilderError {
    /// Uninitialized field
    UninitializedField(String),
    /// Custom validation error
    ValidationError(String),
}

impl From<UninitializedFieldError> for BuilderError {
    fn from(ufe: UninitializedFieldError) -> BuilderError {
        BuilderError::UninitializedField(ufe.field_name().to_string())
    }
}

impl From<String> for BuilderError {
    fn from(s: String) -> Self {
        Self::ValidationError(s)
    }
}

impl Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuilderError::UninitializedField(field) => {
                f.write_fmt(format_args!("Uninitialized field {}", field))
            }
            BuilderError::ValidationError(reason) => {
                f.write_fmt(format_args!("Validation error! Reason : {}", reason))
            }
        }
    }
}

impl std::error::Error for BuilderError {}

pub type RawKumaResult<T> = Result<T, Error>;
