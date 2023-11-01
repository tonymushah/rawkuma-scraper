#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone)]
pub enum Type {
    All,
    Manga,
    Manhwa,
    Manhua,
    Comic,
    Novel,
}

impl<'a> Type {
    pub fn as_str(&self) -> &'a str {
        match self {
            Type::All => "",
            Type::Manga => "manga",
            Type::Manhwa => "manhwa",
            Type::Manhua => "manhua",
            Type::Comic => "comic",
            Type::Novel => "novel",
        }
    }
}

impl<'a> AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for Type {
    fn from(value: &'a str) -> Self {
        match value {
            "manga" => Type::Manga,
            "manhwa" => Type::Manhwa,
            "manhua" => Type::Manhua,
            "comic" => Type::Comic,
            "novel" => Type::Novel,
            _ => Default::default(),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::All
    }
}
