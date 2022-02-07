use colored::*;
use serde::Deserialize;
use strum::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum FormatRadix {
    Decimal,
    Hex,
    Octal,
    Binary,
}

impl FormatRadix {
    pub fn name(&self) -> String {
        match self {
            FormatRadix::Decimal => "Decimal".green().to_string(),
            FormatRadix::Hex => "Hexadecimal".yellow().to_string(),
            FormatRadix::Octal => "Octal".blue().to_string(),
            FormatRadix::Binary => "Binary".magenta().to_string(),
        }
    }
}

impl Default for FormatRadix {
    fn default() -> Self {
        Self::Hex
    }
}

#[derive(Default)]
pub struct OutputFormat {
    radix: FormatRadix,
}

impl OutputFormat {
    pub fn with_format_radix(mut self, radix: FormatRadix) -> Self {
        self.radix = radix;
        self
    }

    pub fn set_format_radix(&mut self, radix: FormatRadix) {
        self.radix = radix;
    }

    pub fn fmt(&self, num: i64) -> String {
        match self.radix {
            FormatRadix::Decimal => format!("{}", num),
            FormatRadix::Hex => format!("{:#x}", num),
            FormatRadix::Octal => format!("{:#o}", num),
            FormatRadix::Binary => format!("{:#b}", num),
        }
    }
}
