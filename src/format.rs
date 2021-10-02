use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum FormatStyle {
    Decimal,
    Hex,
    Octal,
    Binary,
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
