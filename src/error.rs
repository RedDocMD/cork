use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProccessCommandError {
    #[error("Couldn't evaluate the expressison")]
    Evaluation,
    #[error("Invalid {value:?} value for key {key:?}")]
    InvalidValueForKey { value: String, key: String },
    #[error("{0} is not a valid key")]
    InvalidKey(String),
    #[error("Parsing Error")]
    Parsing,
}
