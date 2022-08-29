use std::{
    char,
    fmt::{self, Display, Formatter},
};

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
    fn fmt_uint_to_chars(&self, num: u64) -> Vec<char> {
        let mut rev_chars = match self {
            FormatRadix::Decimal => uint_to_chars_radix(num, 10),
            FormatRadix::Hex => uint_to_chars_radix(num, 16),
            FormatRadix::Octal => uint_to_chars_radix(num, 8),
            FormatRadix::Binary => uint_to_chars_radix(num, 2),
        };
        rev_chars.reverse();
        rev_chars
    }
}

impl From<FormatRadix> for u32 {
    fn from(val: FormatRadix) -> Self {
        match val {
            FormatRadix::Decimal => 10,
            FormatRadix::Hex => 16,
            FormatRadix::Octal => 8,
            FormatRadix::Binary => 2,
        }
    }
}

impl Display for FormatRadix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FormatRadix::Decimal => write!(f, "{}", "Decimal".green()),
            FormatRadix::Hex => write!(f, "{}", "Hexadecimal".yellow()),
            FormatRadix::Octal => write!(f, "{}", "Octal".blue()),
            FormatRadix::Binary => write!(f, "{}", "Binary".magenta()),
        }
    }
}

fn uint_to_chars_radix(mut num: u64, radix: u32) -> Vec<char> {
    let mut chars = Vec::new();
    if num == 0 {
        chars.push('0');
    }
    while num > 0 {
        let d = (num % radix as u64) as u32;
        chars.push(char::from_digit(d, radix).unwrap());
        num /= radix as u64;
    }
    chars
}

fn uint_with_separators(chars: &[char], radix: FormatRadix) -> String {
    let interval = match radix {
        FormatRadix::Decimal | FormatRadix::Octal => 3,
        FormatRadix::Hex | FormatRadix::Binary => 4,
    };
    let chunks: Vec<_> = chars
        .rchunks(interval)
        .map(String::from_iter)
        .rev()
        .collect();
    chunks.join("_")
}

impl Default for FormatRadix {
    fn default() -> Self {
        Self::Hex
    }
}

#[derive(Default)]
pub struct OutputFormat {
    radix: FormatRadix,
    punctuate_number: bool,
}

impl OutputFormat {
    pub fn with_format_radix(mut self, radix: FormatRadix) -> Self {
        self.radix = radix;
        self
    }

    pub fn with_punctuate_number(mut self, punctuate_number: bool) -> Self {
        self.punctuate_number = punctuate_number;
        self
    }

    pub fn set_format_radix(&mut self, radix: FormatRadix) {
        self.radix = radix;
    }

    pub fn fmt(&self, num: i64) -> String {
        let (abs_num, negative) = if num < 0 {
            (-num as u64, true)
        } else {
            (num as u64, false)
        };
        let abs_num_chars = self.radix.fmt_uint_to_chars(abs_num);
        let abs_num_str = if self.punctuate_number {
            uint_with_separators(&abs_num_chars, self.radix)
        } else {
            String::from_iter(&abs_num_chars)
        };
        let prefix = match self.radix {
            FormatRadix::Decimal => "",
            FormatRadix::Hex => "0x",
            FormatRadix::Octal => "0o",
            FormatRadix::Binary => "0b",
        };

        if negative {
            format!("-{}{}", prefix, abs_num_str)
        } else {
            format!("{}{}", prefix, abs_num_str)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0_fmt() {
        let cases = [
            (FormatRadix::Hex, "0x0"),
            (FormatRadix::Octal, "0o0"),
            (FormatRadix::Binary, "0b0"),
            (FormatRadix::Decimal, "0"),
        ];

        for (radix, output) in cases {
            let mut of = OutputFormat::default().with_format_radix(radix);

            // Regardless of punctuation, output should be the same
            of = of.with_punctuate_number(false);
            assert_eq!(of.fmt(0), output);

            of = of.with_punctuate_number(true);
            assert_eq!(of.fmt(0), output);
        }
    }
}
