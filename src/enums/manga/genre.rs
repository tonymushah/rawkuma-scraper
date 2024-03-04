use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone)]
pub enum Genre {
    All,
    Action,
    Adult,
    Adventure,
    #[cfg_attr(feature = "serde", serde(rename = "Alternative_World"))]
    AlternativeWorld,
    Comedy,
    Drama,
    Ecchi,
    Elves,
    English,
    Fantasy,
    Food,
    Game,
    #[cfg_attr(feature = "serde", serde(rename = "Gender_Bender"))]
    GenderBender,
    Harem,
    Historical,
    Horror,
    Isekai,
    Josei,
    Lolicon,
    Magic,
    #[cfg_attr(feature = "serde", serde(rename = "Martial_Arts"))]
    MartialArts,
    Mature,
    Mecha,
    Medical,
    Mystery,
    #[cfg_attr(feature = "serde", serde(rename = "N_A"))]
    NA,
    Oneshot,
    Psychological,
    Romance,
    #[cfg_attr(feature = "serde", serde(rename = "School_Life"))]
    SchoolLife,
    #[cfg_attr(feature = "serde", serde(rename = "Sci_Fi"))]
    SciFi,
    Seinen,
    Shotacon,
    Shoujo,
    #[cfg_attr(feature = "serde", serde(rename = "Shoujo_Ai"))]
    ShoujoAi,
    Shounen,
    #[cfg_attr(feature = "serde", serde(rename = "Shounen_Ai"))]
    ShounenAi,
    #[cfg_attr(feature = "serde", serde(rename = "Slice_Of_Life"))]
    SliceOfLife,
    Smut,
    Sports,
    Supernatural,
    Tragedy,
    Updating,
    War,
    Yaoi,
    Yuri,
}
impl<'a> Genre {
    pub fn as_str(&self) -> &'a str {
        match self {
            Genre::All => "",
            Genre::Action => "action",
            Genre::Adult => "adult",
            Genre::Adventure => "adventure",
            Genre::AlternativeWorld => "alternative-world",
            Genre::Comedy => "comedy",
            Genre::Drama => "drama",
            Genre::Ecchi => "ecchi",
            Genre::Elves => "elves",
            Genre::English => "english",
            Genre::Fantasy => "fantasy",
            Genre::Food => "food",
            Genre::Game => "game",
            Genre::GenderBender => "gender-bende",
            Genre::Harem => "harem",
            Genre::Historical => "historical",
            Genre::Horror => "horror",
            Genre::Isekai => "isekai",
            Genre::Josei => "josei",
            Genre::Lolicon => "lolicon",
            Genre::Magic => "magic",
            Genre::MartialArts => "martial-arts",
            Genre::Mature => "mature",
            Genre::Mecha => "mecha",
            Genre::Medical => "medical",
            Genre::Mystery => "mystery",
            Genre::NA => "n-a",
            Genre::Oneshot => "oneshot",
            Genre::Psychological => "psychological",
            Genre::Romance => "romance",
            Genre::SchoolLife => "school-life",
            Genre::SciFi => "sci-fi",
            Genre::Seinen => "seinen",
            Genre::Shotacon => "shotacon",
            Genre::Shoujo => "shoujo",
            Genre::ShoujoAi => "shoujo-ai",
            Genre::Shounen => "shounen",
            Genre::ShounenAi => "shounen-ai",
            Genre::SliceOfLife => "slice-of-life",
            Genre::Smut => "smut",
            Genre::Sports => "sports",
            Genre::Supernatural => "supernatural",
            Genre::Tragedy => "tragedy",
            Genre::Updating => "updating",
            Genre::War => "war",
            Genre::Yaoi => "yaoi",
            Genre::Yuri => "yuri",
        }
    }
}

impl AsRef<str> for Genre {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for Genre {
    fn from(value: &'a str) -> Self {
        match value {
            "adult" => Self::Adult,
            "action" => Self::Action,
            "adventure" => Self::Adventure,
            "alternative-world" => Self::AlternativeWorld,
            "comedy" => Self::Comedy,
            "drama" => Self::Drama,
            "ecchi" => Self::Ecchi,
            "elves" => Self::Elves,
            "english" => Self::English,
            "fantasy" => Self::Fantasy,
            "food" => Self::Food,
            "game" => Self::Game,
            "gender-bende" => Self::GenderBender,
            "harem" => Self::Harem,
            "historical" => Self::Historical,
            "horror" => Self::Horror,
            "isekai" => Self::Isekai,
            "josei" => Self::Josei,
            "lolicon" => Self::Lolicon,
            "magic" => Self::Magic,
            "martial-arts" => Self::MartialArts,
            "mature" => Self::Mature,
            "mecha" => Self::Mecha,
            "medical" => Self::Medical,
            "mystery" => Self::Mystery,
            "n-a" => Self::NA,
            "oneshot" => Self::Oneshot,
            "psychological" => Self::Psychological,
            "romance" => Self::Romance,
            "school-life" => Self::SchoolLife,
            "sci-fi" => Self::SciFi,
            "seinen" => Self::Seinen,
            "shotacon" => Self::Shotacon,
            "shoujo" => Self::Shoujo,
            "shoujo-ai" => Self::ShoujoAi,
            "shounen" => Self::Shounen,
            "shounen-ai" => Self::ShounenAi,
            "slice-of-life" => Self::SliceOfLife,
            "smut" => Self::Smut,
            "sports" => Self::Sports,
            "supernatural" => Self::Supernatural,
            "tragedy" => Self::Tragedy,
            "updating" => Self::Updating,
            "war" => Self::War,
            "yaoi" => Self::Yaoi,
            "yuri" => Self::Yuri,
            _ => Self::All,
        }
    }
}

impl Default for Genre {
    fn default() -> Self {
        Self::All
    }
}
