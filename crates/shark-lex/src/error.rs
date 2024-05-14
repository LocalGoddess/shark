use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct UnknownNumericSuffixError {
    pub invalid_suffix: String,
}

impl Display for UnknownNumericSuffixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown numeric suffix: {}", self.invalid_suffix)
    }
}

impl Error for UnknownNumericSuffixError {}

#[derive(Debug, Default)]
pub struct InvalidFloatRadix;

impl Display for InvalidFloatRadix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "float literals must be written in base10")
    }
}

impl Error for InvalidFloatRadix {}
