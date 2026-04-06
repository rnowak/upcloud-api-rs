# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - Unreleased

### Added

- Full coverage of all 22 UpCloud API resource groups
- Domain-scoped handler pattern: `client.servers().list(...)`, `client.managed_database().create(...)`
- Bearer token and HTTP Basic authentication
- Custom `reqwest::Client` support via `ClientBuilder::http_client()`
- Typed query parameters for filtering, ordering, and pagination
- Strongly-typed request/response structs with serde
- Async/await throughout (tokio)
- TLS via rustls (no OpenSSL dependency)
- `reqwest` re-exported for version compatibility
- Comprehensive test suite (321 tests) using real UpCloud API doc fixtures
