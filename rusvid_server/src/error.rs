use std::sync::PoisonError;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::Display;
use serde_json::json;
use thiserror::Error;

use crate::render_task::Message;

#[derive(Debug, Error, Display)]
pub enum ApiError {
    FileNotFound,
    HeaderParseError(#[from] axum::http::header::InvalidHeaderValue),
    IoError(#[from] std::io::Error),
    LockError,
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    NotFound,
    ObjectStorageError(#[from] s3::error::S3Error),
    RedisError(#[from] redis::RedisError),
    R2D2Error(#[from] r2d2::Error),
    SendError(#[from] tokio::sync::mpsc::error::SendError<Message>),
    UnknownError,
    VideoEncounteredError,
    VideoInProcess,
    YamlDeserializeError(#[from] serde_yaml::Error),
}

impl ApiError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            ApiError::FileNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::HeaderParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::LockError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MultipartError(err) => err.status(),
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ObjectStorageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::R2D2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::VideoEncounteredError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::VideoInProcess => StatusCode::PROCESSING,
            ApiError::YamlDeserializeError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn get_message(&self) -> String {
        match self {
            ApiError::UnknownError => "An internal server error occurred.".to_string(),
            ApiError::LockError => "An internal server error occurred. (LockError)".to_string(),
            ApiError::NotFound => "Oops! We can't find what you are searching fore!".to_string(),
            ApiError::FileNotFound => {
                "No multipart upload with name 'file' has been found.".to_string()
            }
            ApiError::SendError(_) => "An internal server error occurred. (SendError)".to_string(),
            ApiError::YamlDeserializeError(err) => {
                println!("{err:?}");
                "Error while parsing YAML file.".to_string()
            }
            ApiError::MultipartError(err) => err.body_text(),
            ApiError::VideoInProcess => {
                "Video is still being processed. You have to wait a little bit longer".to_string()
            }
            ApiError::IoError(err) => {
                println!("{err:?}");
                "An internal server error occurred. (IoError)".to_string()
            }
            ApiError::ObjectStorageError(err) => {
                println!("{err:?}");
                "An internal server error occurred. (ObjectStorageError)".to_string()
            }
            ApiError::R2D2Error(err) => {
                println!("{err:?}");
                "An internal server error occurred. (RedisR2D2Error)".to_string()
            }
            ApiError::RedisError(err) => {
                println!("{err:?}");
                "An internal server error occurred. (RedisError)".to_string()
            }
            ApiError::HeaderParseError(err) => {
                println!("{err:?}");
                "An internal server error occurred. (HeaderParseError)".to_string()
            }
            ApiError::VideoEncounteredError => "Composition encountered an error".to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.to_status_code();
        let message = self.get_message();

        let body = json!({
            "message": message,
            "status": status.as_str()
        });

        (status, Json(body)).into_response()
    }
}

impl<T> From<PoisonError<T>> for ApiError {
    fn from(_: PoisonError<T>) -> Self {
        ApiError::LockError
    }
}
