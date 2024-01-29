use serde::{Deserialize, Serialize};

pub mod http;
pub mod config;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub id: Option<i32>,
    pub name: String,
    pub color: Color,
    pub owner: String,
    pub portrait: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        let value = value.trim_start_matches('#');

        let red = u8::from_str_radix(&value[0..2], 16).unwrap();
        let green = u8::from_str_radix(&value[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&value[4..6], 16).unwrap();
        let alpha = u8::from_str_radix(&value[6..8], 16).unwrap();

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
