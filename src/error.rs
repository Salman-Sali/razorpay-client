use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while calling api : {0}")]
    ApiError(String),
    /// Razorpay rejected the request; `error` is the parsed `error` object of the response.
    #[error("Razorpay error : {context}, StatusCode: {status}, Error: {error}")]
    Razorpay {
        status: u16,
        context: String,
        error: Box<RazorpayError>,
    },
    #[error("Error while parsing url : {0}")]
    UrlParseError(url::ParseError),
    #[error("{0}")]
    SerdeJsonError(serde_json::Error),
    #[error("Something went wrong : {0}")]
    SomethingWentWrong(String),
}

impl Error {
    pub(crate) fn from_response(status: u16, context: String, body: String) -> Self {
        #[derive(serde::Deserialize)]
        struct ErrorResponse {
            error: RazorpayError,
        }

        match serde_json::from_str::<ErrorResponse>(&body) {
            Ok(response) => Self::Razorpay {
                status,
                context,
                error: Box::new(response.error),
            },
            Err(_) => Self::ApiError(format!("{context}, StatusCode: {status}, Error: {body:?}")),
        }
    }
}

/// The `error` object of a failed Razorpay response.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RazorpayError {
    /// e.g. `BAD_REQUEST_ERROR`, `GATEWAY_ERROR`, `SERVER_ERROR`.
    pub code: String,
    pub description: Option<String>,
    pub source: Option<String>,
    pub step: Option<String>,
    pub reason: Option<String>,
    pub field: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl Display for RazorpayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)?;
        if let Some(description) = &self.description {
            write!(f, ": {description}")?;
        }
        if let Some(field) = &self.field {
            write!(f, " (field: {field})")?;
        }
        Ok(())
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_razorpay_error_body() {
        let error = Error::from_response(
            400,
            "OrderId: order_1".into(),
            r#"{"error":{"code":"BAD_REQUEST_ERROR","description":"The id provided does not exist","source":"business","step":"payment_initiation","reason":"input_validation_failed","metadata":{},"field":"id"}}"#.into(),
        );

        let Error::Razorpay { status, error, .. } = error else {
            panic!("expected Razorpay error");
        };
        assert_eq!(status, 400);
        assert_eq!(error.code, "BAD_REQUEST_ERROR");
        assert_eq!(error.field.as_deref(), Some("id"));
    }

    #[test]
    fn should_fall_back_to_api_error() {
        let error = Error::from_response(502, "ctx".into(), "Bad Gateway".into());

        assert!(matches!(error, Error::ApiError(e) if e.contains("502")));
    }
}
