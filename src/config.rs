use std::{fs::File, io::Read, path::PathBuf};

use crate::format::FormatStyle;
use anyhow::Result as AResult;
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

fn config_locations() -> Vec<PathBuf> {
    match home::home_dir() {
        Some(home) => {
            let mut at_home = home.clone();
            at_home.push(".cork.yml");
            let mut at_cork = home.clone();
            at_cork.push(".cork");
            at_cork.push("cork.yml");
            let mut at_config = home.clone();
            at_config.push(".config");
            at_config.push("cork");
            at_config.push("cork.yml");
            vec![at_home, at_cork, at_config]
        }
        None => Vec::new(),
    }
}

pub fn read_config() -> AResult<Config> {
    let locations = config_locations();
    let mut content = String::new();
    for loc in &locations {
        if loc.exists() && loc.is_file() {
            let mut file = File::open(loc)?;
            file.read_to_string(&mut content)?;
        }
    }
    if content.is_empty() {
        content = String::from("[]")
    }
    let config = serde_yaml::from_str(&content)?;
    Ok(config)
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

    #[test]
    fn test_config_deserialize_missing_values() {
        let config_str = "prompt: $
output_radix: Octal";
        let config: Config = serde_yaml::from_str(config_str).unwrap();
        let expected_config = Config {
            prompt: String::from("$"),
            header: default_header(),
            output_radix: FormatStyle::Octal,
        };
        assert_eq!(config, expected_config);
    }

    #[test]
    fn test_config_deserialize_empty() {
        let config_str = "[]";
        let config: Config = serde_yaml::from_str(config_str).unwrap();
        let expected_config = Config {
            prompt: default_prompt(),
            header: default_header(),
            output_radix: FormatStyle::default(),
        };
        assert_eq!(config, expected_config);
    }
}
