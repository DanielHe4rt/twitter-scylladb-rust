pub mod controllers;

use actix_web::{HttpResponse, ResponseError};
use charybdis::errors::CharybdisError;
use serde_json::json;
use thiserror::Error;



#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Charybdis error: {0}")]
    CharybdisError(#[from] CharybdisError),
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::CharybdisError(e) => match e {
                CharybdisError::NotFoundError(e) => HttpResponse::NotFound().json(json!({
                    "status": 404,
                    "message": e.to_string()
                })),
                _ => HttpResponse::InternalServerError().json(json!({
                    "status": 500,
                    "message": e.to_string()
                }))
            }
        }
    }
}