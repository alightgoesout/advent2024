use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct Error(pub String);

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self(format!("IO error: {value}"))
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self(format!("Error while parsing integer: {value}"))
    }
}
