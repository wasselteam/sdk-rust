use wassel_sdk::http::{IntoResponse, Request, StatusCode, handler};

#[handler]
fn handle_request(request: Request) -> impl IntoResponse {
    match request.uri().path() {
        "/secret" => (StatusCode::OK, get_config("secret")),
        "/host" => (StatusCode::OK, get_config("host")),
        "/hostname" => (StatusCode::OK, get_config("hostname")),
        _ => (StatusCode::NOT_FOUND, "".to_owned()),
    }
}

fn get_config(key: &str) -> String {
    wassel_sdk::bindings::wasi_config::store::get(key)
        .unwrap_or_default()
        .unwrap_or_default()
}
