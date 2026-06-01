use std::error::Error;

use wassel_sdk::{
    bindings::wassel::postgres::postgres::{self, Parameter},
    http::{IntoResponse, Request, StatusCode, handler},
};

// See `https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html`
// for connection string options
const CONNECTION_STRING: &str =
    "host=127.0.0.1 port=5432 user=postgres password=MegaVeryStongPass1337 dbname=digital-deanery";

// This functiuon demonstrates how to query the database
fn make_query() -> Result<i64, Box<dyn Error>> {
    // Create default config using connection string
    let config = postgres::ConnectionConfig::new(CONNECTION_STRING);

    // Try to open the connection using config
    let conn = postgres::Connection::open(config)?;

    // Execute query and get results
    let rows = conn.query(
        // NOTE: you don't have to annotate types in SQL since they are
        // inferred from parameters
        "SELECT $1 + $2",
        &[Parameter::Int64(34), Parameter::Int64(35)],
    )?;

    // Try to get the first row and then get the first value from that row.
    // We expect query to return a single row with type `(INT8)`
    let Some(postgres::Value::Int64(num)) = rows.rows.first().and_then(|r| r.first()) else {
        return Err("Expected Int64 in row".to_owned().into());
    };

    // Return result
    Ok(*num)
}

// Implement WASM exports
#[handler]
fn handle_request(_request: Request) -> impl IntoResponse {
    match make_query() {
        Ok(num) => (StatusCode::OK, format!("SELECT 1 + 1 = {num}")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
