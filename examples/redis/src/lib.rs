use std::error::Error;

use wassel_sdk::{
    bindings::wassel::redis::redis::{self, RedisArgument, RedisValue},
    http::{IntoResponse, Request, StatusCode, handler},
};

const CONNECTION_STRING: &str = "redis://localhost:6379";

// This functiuon demonstrates how to query the database
fn make_query() -> Result<String, Box<dyn Error>> {
    // Create default config using connection string
    let config = redis::ConnectionConfig::new(CONNECTION_STRING);

    // Try to open the connection using config
    let conn = redis::Connection::open(config)?;

    // Set key "my:value" to value "Hello"
    conn.execute(
        "SET",
        &[
            RedisArgument::Str("my:value".to_owned()),
            RedisArgument::Str("Hello".to_owned()),
        ],
    )?;

    // Retrieve back the value of key "my:value"
    let str = match conn.execute("GET", &[RedisArgument::Str("my:value".to_owned())])? {
        RedisValue::BulkString(s) => String::from_utf8_lossy(&s).to_string(),
        other => return Err(format!("Unexpected value `{other:?}`").into()),
    };

    Ok(str)
}

// Implement WASM exports
#[handler]
fn handle_request(_request: Request) -> impl IntoResponse {
    match make_query() {
        Ok(str) => (StatusCode::OK, format!("my:value = {str}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
