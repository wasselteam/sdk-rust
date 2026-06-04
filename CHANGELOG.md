# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - yyyy-mm-dd

### Changed

- [sdk] BREAKING: Renamed `postgres::Value::Unsupported` to `postgres::Value::Other`
- [sdk] BREAKING: Update WIT with new `wassel:postgres` and `wassel:http-client` packages
- [sdk] Update WIT with new `wassel:redis` package

### Added

- [example] Added axum example
- [example] Added fibonacci sequence example
- [example] Added redis example

## [2.1.2] - 2026.05.20

### Fixed

- [framework] `Body` `Read` impl hangs (https://github.com/wasselteam/sdk-rust/issues/1)

### Added

- [framework] `From<Bytes>` implementation for Body

### Changed

- [examples] Make examples members of workspace
- [framework] Remove `Read` implementation from body

## [2.1.1] - 2026.05.12

### Fixed

- [framework] Fix panic in Read impl of Body

## [2.1.0] - 2026.05.12

### Added

- [framework] Introduce HTTP client to simplify making HTPT requests

### Changed

- [example] Update HTTP example to use new HTTP client

## [2.0.0] - 2026.05.12

### Added

- [framework] `handler` attribute macro
- [framework] `Body`, `Request`, `Response`, `IntoResponse` types

### Changed

- [sdk] BREAKING: Renamed package `wassel-sdk-rust` to `wassel-sdk`
- [sdk] BREAKING: Renamed `wassel_sdk::bindings::wasi::config` to `wassel_sdk::bindings::wasi_config`

## [1.0.0] - 2026-05-11

### Added

- [sdk] Implement raw bindings
- [example] Update examples according to wassel build facilities
- [example] Add HTTP-client example
- [example] Add data-folder example
- [example] Add postgres example
- [example] Add configuration example

### Changed

- [sdk] Use wasip2 bindings instead of generating
