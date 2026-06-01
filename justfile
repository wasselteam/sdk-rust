@_default:
    just --list

prepare:
    cargo clippy --fix --allow-dirty -- -D warnings
    cargo test
    cargo fmt

wit:
    wkg wit fetch -d wit -t wit
