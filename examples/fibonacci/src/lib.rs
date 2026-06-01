use wassel_sdk::http::{IntoResponse, Request, StatusCode, handler};

#[handler]
fn handler(request: Request) -> impl IntoResponse {
    let mut path = request.uri().path();
    if path.starts_with("/") {
        path = &path[1..];
    }

    let Ok(num) = path.parse() else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let result = fib(num);

    format!("Result: {}", result).into_response()
}

fn fib(n: u64) -> u64 {
    if n <= 2 { 1 } else { fib(n - 1) + fib(n - 2) }
}
