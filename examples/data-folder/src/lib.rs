use std::fs;
use wassel_sdk::http::{IntoResponse, Request, StatusCode, handler};

#[handler]
fn handle_request(_request: Request) -> impl IntoResponse {
    match fs::read("file.txt") {
        Ok(data) => (StatusCode::OK, data),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().as_bytes().to_vec(),
        ),
    }
}
