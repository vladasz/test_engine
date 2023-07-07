use std::{
    error::Error,
    fmt::{Display, Formatter},
    sync::PoisonError,
};

use actix_web::ResponseError;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestError {
    pub message: String,
}

impl Error for RestError {}

impl From<RestError> for String {
    fn from(value: RestError) -> Self {
        value.message
    }
}

impl From<String> for RestError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&'static str> for RestError {
    fn from(str: &'static str) -> Self {
        Self { message: str.into() }
    }
}

impl From<reqwest::Error> for RestError {
    fn from(error: reqwest::Error) -> Self {
        format!("Network error: {error}").into()
    }
}

impl From<serde_json::Error> for RestError {
    fn from(err: serde_json::Error) -> Self {
        format!("Serialization error: {err}").into()
    }
}

impl<T> From<PoisonError<T>> for RestError {
    fn from(err: PoisonError<T>) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<sea_orm::DbErr> for RestError {
    fn from(err: DbErr) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl Display for RestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

pub type NetResult<T> = Result<T, RestError>;

impl ResponseError for RestError {}
