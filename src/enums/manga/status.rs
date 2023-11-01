#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone)]
pub enum Status {
    All,
    Ongoing,
    Completed,
    Hiatus,
}

impl Status {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Status::All => "",
            Status::Ongoing => "ongoing",
            Status::Completed => "completed",
            Status::Hiatus => "hiatus",
        }
    }
}

impl<'a> AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for Status {
    fn from(value: &'a str) -> Self {
        match value {
            "ongoing" => Self::Ongoing,
            "completed" => Self::Completed,
            "hiatus" => Self::Hiatus,
            _ => Self::All,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::All
    }
}
