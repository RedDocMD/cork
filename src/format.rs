pub enum FormatStyle {
    Decimal,
    Hex,
    Octal,
    Binary,
}

pub struct OutputFormat {
    style: FormatStyle,
}

impl OutputFormat {
    pub fn default() -> Self {
        Self {
            style: FormatStyle::Hex,
        }
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
