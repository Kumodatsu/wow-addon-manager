use std::io;
use reqwest;

#[derive(Debug)]
pub enum AddonError {
    IOError(io::Error),
    RequestError(reqwest::Error),
}

impl From<io::Error> for AddonError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<reqwest::Error> for AddonError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}
