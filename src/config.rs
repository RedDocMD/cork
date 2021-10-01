use crate::format::FormatStyle;
use getset::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize, Getters, PartialEq, Eq)]
#[getset(get = "pub")]
pub struct Config {
    #[serde(default = "default_prompt")]
    prompt: String,

    #[serde(default = "default_header")]
    header: bool,

    #[serde(default)]
    output_radix: FormatStyle,
}

fn default_prompt() -> String {
    String::from("cork>")
}

fn default_header() -> bool {
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_deserialize() {
        let config_str = "prompt: $
header: false
output_radix: Octal";
        let config: Config = serde_yaml::from_str(config_str).unwrap();
        let expected_config = Config {
            prompt: String::from("$"),
            header: false,
            output_radix: FormatStyle::Octal,
        };
        assert_eq!(config, expected_config);
    }
}
