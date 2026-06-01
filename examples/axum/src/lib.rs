use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;
use wassel_sdk::http::{Body, IntoResponse, Request, Response, handler};

#[derive(Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
    age: u16,
}

#[handler]
fn handler(request: Request) -> impl IntoResponse {
    let request = convert_request(request);

    let app = axum::Router::new()
        .route("/hello", axum::routing::get(hello))
        .route("/persons", axum::routing::post(post_person))
        .route("/persons/{id}", axum::routing::get(get_person_id));

    pollster::block_on(async move {
        let response = app.oneshot(request).await.expect("Infallible");
        convert_response(response).await
    })
}

async fn hello() -> impl axum::response::IntoResponse {
    "Hello!"
}

async fn post_person(Json(person): Json<Person>) -> impl axum::response::IntoResponse {
    Json(person)
}

async fn get_person_id(Path(id): Path<i32>) -> impl axum::response::IntoResponse {
    Json(Person {
        id,
        name: "Wasm".to_owned(),
        age: 9,
    })
}

fn convert_request(request: Request) -> axum::http::Request<axum::body::Body> {
    let (parts, body) = request.into_parts();
    let body = match body {
        wassel_sdk::http::Body::Empty => axum::body::Body::empty(),
        wassel_sdk::http::Body::Full(items) => axum::body::Body::from(items),
        wassel_sdk::http::Body::Stream(mut read) => {
            let mut bytes = Vec::new();
            let _ = read.read_to_end(&mut bytes);
            axum::body::Body::from(bytes)
        }
    };
    axum::http::Request::<axum::body::Body>::from_parts(parts, body)
}

async fn convert_response(response: axum::response::Response) -> Response {
    let (parts, body) = response.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .unwrap_or_default();
    let body = Body::from(bytes.to_vec());
    Response::from_parts(parts, body)
}
