#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Couldn't fetch resources")]
    NetworkError,

    #[error("Error parsing resource")]
    ParseError,
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Self::ParseError
    }
}

impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(_: scraper::error::SelectorErrorKind) -> Self {
        Self::ParseError
    }
}
