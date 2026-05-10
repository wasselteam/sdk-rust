mod example-configuration "examples/configuration/justfile"
mod example-data-folder "examples/data-folder/justfile"
mod example-hello "examples/hello/justfile"
mod example-http-request "examples/http-request/justfile"
mod example-postgres "examples/postgres/justfile"

@_default:
    just --list

prepare:
    cargo clippy --fix --allow-dirty -- -D warnings
    cargo test
    cargo fmt

    just example-configuration::prepare
    just example-data-folder::prepare
    just example-hello::prepare
    just example-http-request::prepare
    just example-postgres::prepare

wit:
    wkg wit fetch -d wit -t wit
