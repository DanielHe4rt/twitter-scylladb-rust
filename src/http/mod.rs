use actix_web::{HttpResponse, ResponseError};
use charybdis::errors::CharybdisError;
use serde::Serialize;

use crate::Error;

pub mod controllers;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: std::borrow::Cow<'static, str>,
}

impl ResponseError for crate::Error {
    fn error_response(&self) -> HttpResponse {
        // TODO: Logging
        match self {
            Self::CharybdisError(e) => match e {
                CharybdisError::NotFoundError(_) => HttpResponse::NotFound().json(ErrorResponse{
                    message: "Record not found".into()
                }),
                _ => HttpResponse::InternalServerError().json(ErrorResponse{
                    message: "Server Error".into()
                })
            },
            Error::QueryError(_) => HttpResponse::InternalServerError().json(ErrorResponse{
                message: "Server Error".into()
            }),

            Error::NotFound => HttpResponse::NotFound().json(ErrorResponse{
                message: "Record not found".into()
            }),
        }
    }
}