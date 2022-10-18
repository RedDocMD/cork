use crate::expression::PestRuleError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CorkError {
    #[error("couldn't evaluate the expressison: {0}")]
    Eval(String),
    #[error("invalid {value} value for key {key}")]
    InvalidValueForKey { value: String, key: String },
    #[error("{0} is not a valid key")]
    InvalidKey(String),
    #[error("parsing error:\n{0}")]
    Parse(#[from] PestRuleError),
    #[error("{0} is not a valid destination radix")]
    InvalidConversion(String),
}
