use colored::*;
use serde::Deserialize;
use strum::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum FormatStyle {
    Decimal,
    Hex,
    Octal,
    Binary,
}

impl FormatStyle {
    pub fn name(&self) -> String {
        match self {
            FormatStyle::Decimal => "Decimal".green().to_string(),
            FormatStyle::Hex => "Hexadecimal".yellow().to_string(),
            FormatStyle::Octal => "Octal".blue().to_string(),
            FormatStyle::Binary => "Binary".magenta().to_string(),
        }
    }
}

impl Default for FormatStyle {
    fn default() -> Self {
        Self::Hex
    }
}

pub struct OutputFormat {
    style: FormatStyle,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self {
            style: FormatStyle::Hex,
        }
    }
}

impl OutputFormat {
    pub fn from_format_style(f: FormatStyle) -> Self {
        Self { style: f }
    }

    pub fn set_format_style(&mut self, f: FormatStyle) {
        self.style = f;
    }
}

pub fn fmt(num: i64, f: &OutputFormat) -> String {
    match f.style {
        FormatStyle::Decimal => format!("{}", num),
        FormatStyle::Hex => format!("{:#x}", num),
        FormatStyle::Octal => format!("{:#o}", num),
        FormatStyle::Binary => format!("{:#b}", num),
    }
}
