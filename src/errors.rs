use confy::ConfyError;
use reqwest::Error as ReqwestError;
use std::fmt;
use url::ParseError as UrlError;

#[derive(Debug)]
pub enum AppError {
    ConfigSetup(ConfyError),
    ApiRequest(ReqwestError),
    UrlParse(UrlError),
    TimeParse(String),
    Coordinates(CoordinatesError),
}

#[derive(Debug)]
pub enum CoordinatesError {
    Latitude(f32),
    Longitude(f32),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigSetup(e) => writeln!(f, "Config setup error: {}", e),
            AppError::ApiRequest(e) => writeln!(f, "Api request error: {}", e),
            AppError::UrlParse(e) => writeln!(f, "Url parse error: {}", e),
            AppError::TimeParse(e) => writeln!(f, "Failed to parse time: {}", e),
            AppError::Coordinates(e) => {
                writeln!(f, "Provided coordinate is incorrent, error: {}", e)
            }
        }
    }
}

impl fmt::Display for CoordinatesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoordinatesError::Latitude(v) => write!(
                f,
                "Latitude must be between -90 and 90 degrees. You value is: {v}"
            ),
            CoordinatesError::Longitude(v) => write!(
                f,
                "Longitude must be between -180 and 180 degrees. Your value is: {v}"
            ),
        }
    }
}

impl From<ConfyError> for AppError {
    fn from(value: ConfyError) -> Self {
        AppError::ConfigSetup(value)
    }
}

impl From<ReqwestError> for AppError {
    fn from(value: ReqwestError) -> Self {
        AppError::ApiRequest(value)
    }
}

impl From<UrlError> for AppError {
    fn from(value: UrlError) -> Self {
        AppError::UrlParse(value)
    }
}
