use wassel_sdk::http::{IntoResponse, Request, handler};

#[handler]
fn handler(request: Request) -> impl IntoResponse {
    format!("Hello, {}, from Rust!", request.uri().path())
}
