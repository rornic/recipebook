pub mod data;
pub mod db;

use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
