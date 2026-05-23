use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while calling api : {0}")]
    ApiError(String),
    #[error("Error while parsing url : {0}")]
    UrlParseError(url::ParseError),
    #[error("{0}")]
    SerdeJsonError(serde_json::Error),
    #[error("Something went wrong : {0}")]
    SomethingWentWrong(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ApiError(format!("{:?}", value))
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}
