use serde::{Deserialize, Serialize};

impl Order {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Order::Default => "",
            Order::AZ => "title",
            Order::ZA => "titlereverse",
            Order::Update => "update",
            Order::Added => "latest",
            Order::Popular => "popular",
        }
    }
}

impl AsRef<str> for Order {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for Order {
    fn from(value: &'a str) -> Self {
        match value {
            "title" => Order::AZ,
            "titlereverse" => Order::ZA,
            "update" => Order::Update,
            "latest" => Order::Added,
            "popular" => Order::Popular,
            _ => Order::Default,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Order {
    Default,
    AZ,
    ZA,
    Update,
    Added,
    Popular,
}

impl Default for Order {
    fn default() -> Self {
        Self::Default
    }
}