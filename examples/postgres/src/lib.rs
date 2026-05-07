use wassel_sdk_rust::bindings::{
    export,
    exports::wassel::foundation::http_handler::Guest,
    wasi::http::types::{
        Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
    },
    wassel::foundation::postgres::{self, Parameter},
};

struct Plugin;

// See `https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html`
// for connection string options
const CONNECTION_STRING: &str =
    "host=127.0.0.1 port=5432 user=postgres password=MegaVeryStongPass1337 dbname=digital-deanery";

// This functiuon demonstrates how to query the database
fn make_query() -> Result<i64, postgres::Error> {
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
    let Some(postgres::Value::Int64(num)) = rows.rows.get(0).and_then(|r| r.get(0)) else {
        return Err(postgres::Error::Query("Expected Int64 in row".to_owned()));
    };

    // Return result
    Ok(*num)
}

// Implement WASM exports
impl Guest for Plugin {
    fn handle_request(_request: IncomingRequest, response_out: ResponseOutparam) {
        let (status, message) = match make_query() {
            Ok(num) => (200, format!("SELECT 1 + 1 = {num}")),
            Err(e) => (500, e.to_string()),
        };

        write_response(response_out, status, Some(message.as_bytes()));
    }
}

// Convenience function to write all bytes to response stream. Please do do
// so many unwraps in your code.
fn write_response(out: ResponseOutparam, status: u16, body_bytes: Option<&[u8]>) {
    let res = OutgoingResponse::new(Headers::new());
    res.set_status_code(status).unwrap();

    if let Some(bytes) = body_bytes {
        let body = res.body().unwrap();
        {
            let stream = body.write().unwrap();
            stream.write(bytes.into()).unwrap();
        }
        OutgoingBody::finish(body, None).unwrap();
    }

    ResponseOutparam::set(out, Ok(res));
}

export!(Plugin);
