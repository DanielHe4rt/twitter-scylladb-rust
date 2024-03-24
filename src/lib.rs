#[warn(async_fn_in_trait)]

use charybdis::errors::CharybdisError;
use charybdis::QueryError;
use thiserror::Error as ThisError;

pub mod models;
pub mod config;
pub mod http;
pub mod repositories;
pub mod services;

pub mod dtos;
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Query error: {0}")]
    QueryError(#[from] QueryError),
    #[error("Charybdis error: {0}")]
    CharybdisError(#[from] CharybdisError),
    #[error("Record not found")]
    NotFound,
}
