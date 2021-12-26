use std::fmt;

#[derive(Debug)]
pub struct InjectorError;

impl fmt::Display for InjectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error :D")
    }
}

impl std::error::Error for InjectorError { }