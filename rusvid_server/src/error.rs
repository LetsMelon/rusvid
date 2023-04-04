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
    UnknownError,
    LockError,
    NotFound,
    FileNotFound,
    SendError(#[from] tokio::sync::mpsc::error::SendError<Message>),
    YamlDeserializeError(#[from] serde_yaml::Error),
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    VideoInProcess,
    IoError(#[from] std::io::Error),
    ObjectStorageError(#[from] s3::error::S3Error),
    RedisR2D2Error(#[from] r2d2_redis::r2d2::Error),
    RedisError(#[from] r2d2_redis::redis::RedisError),
    HeaderParseError(#[from] axum::http::header::InvalidHeaderValue),
}

impl ApiError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            ApiError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::LockError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::FileNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::SendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::YamlDeserializeError(_) => StatusCode::BAD_REQUEST,
            ApiError::MultipartError(err) => err.status(),
            ApiError::VideoInProcess => StatusCode::PROCESSING,
            ApiError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ObjectStorageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::RedisR2D2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::HeaderParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            ApiError::RedisR2D2Error(err) => {
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
