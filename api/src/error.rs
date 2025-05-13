//! Error handling for the API.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use samp_query::Error as QueryError;
use serde::{Deserialize, Serialize};
use std::net::AddrParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to parse socket address: {0}")]
    AddrParse(#[from] AddrParseError),

    #[error("Query error: {0}")]
    Query(#[from] QueryError),

    //note: we're keeping this variant for future use in handling unexpected internal errors
    #[error("Internal server error: {0}")]
    #[allow(dead_code)]
    Internal(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    ///error message.
    pub message: String,
    ///error code.
    pub code: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            ApiError::AddrParse(_) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid server address: {}", self),
            ),
            ApiError::Query(QueryError::Timeout) => (
                StatusCode::GATEWAY_TIMEOUT,
                "Server did not respond in time".to_string(),
            ),
            ApiError::Query(QueryError::Connect(_)) => (
                StatusCode::BAD_GATEWAY,
                "Failed to connect to server".to_string(),
            ),
            ApiError::Query(QueryError::RconAuthFailed) => (
                StatusCode::UNAUTHORIZED,
                "RCON authentication failed".to_string(),
            ),
            ApiError::Query(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Query error: {}", self),
            ),
            ApiError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", self),
            ),
        };

        let body = Json(ErrorResponse {
            message: error_message,
            code: status.as_u16().to_string(),
        });

        (status, body).into_response()
    }
}

