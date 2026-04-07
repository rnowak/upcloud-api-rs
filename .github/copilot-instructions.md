# Copilot Instructions for upcloud-api

Unofficial Rust SDK for the [UpCloud API v1.3](https://developers.upcloud.com/1.3/).

## Commands

```sh
cargo test                                        # All 321 lib tests + doctests
cargo test -- server::tests::deserialize_foo      # Single test by name
cargo test -- server::tests                       # All tests in one module
cargo clippy                                      # Must pass with zero warnings
cargo doc --no-deps                               # Must produce zero warnings
```

## Architecture

**Handler pattern**: Each API domain (servers, storage, networks, etc.) has a handler struct that borrows `&Client`. Created via factory methods: `client.servers()`, `client.load_balancer()`, etc. Handlers are cheap and short-lived.

**Module layout**: Each domain module has:
- `mod.rs` — handler struct + async methods (one per API endpoint)
- `types.rs` — request/response structs, enums (re-exported via `pub use types::*`)
- `tests.rs` — deserialization tests using real UpCloud API doc JSON fixtures

**Response envelope unwrapping**: The UpCloud API wraps responses in envelopes like `{"servers": {"server": [...]}}`. Each handler defines private `#[derive(Deserialize)]` wrapper structs inside `mod.rs` to unwrap these, then returns the inner type.

**Serde quirks**: The API has several non-standard JSON patterns handled by custom helpers in `src/types/common.rs`:
- `YesNo` / `YesNoOpt` — booleans serialized as `"yes"`/`"no"` strings
- `StringNumber` — numbers returned as strings (e.g. `"1024"`)
- `#[serde_as(as = "DisplayFromStr")]` — for numeric fields that come as strings
- Gateway/LB operational states use `#[serde(rename_all = "kebab-case")]`
- Some list endpoints return nested `{"items": {"item": [...]}}` vs flat arrays

**Query parameters**: List endpoints use typed `ListParams` structs with a `pub(crate) fn to_query_pairs()` method. Multi-value params (like `?label=x&label=y`) use `client.get_with_query_pairs()` with explicit `Vec<(&str, String)>`.

## Conventions

**Naming**: Methods follow REST verbs: `list`, `get`, `create`, `modify`, `delete`. Sub-resources use `verb_resource` order: `list_backends`, `create_frontend_rule`, `delete_certificate_bundle`.

**Type naming**: Domain prefixes are dropped inside modules. `LoadBalancerBackend` → `load_balancer::Backend`. The prelude in `lib.rs` re-exports primary types.

**Doc comments on handlers**: Every `pub async fn` must include:
1. Brief description
2. HTTP method + endpoint path in backticks (e.g. `` `GET /1.3/server/{uuid}` ``)
3. Link to UpCloud API docs: `[UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#get-server-details)`

**`#[non_exhaustive]`**: All response/domain structs are annotated `#[non_exhaustive]`. Request, Params, and Filter types are NOT (users construct these with struct literals).

**Test fixtures**: JSON is `const` placed just above the test that uses it, named `DESCRIPTIVE_NAME_JSON`. JSON should be copied from real [UpCloud API doc](https://developers.upcloud.com/1.3/) responses, not fabricated. No trivial enum round-trip tests.

**Section comments**: Code sections are delimited with `// ─── Section Name ───...` box-drawing lines.

**Error handling**: All handler methods return `Result<T, crate::error::Error>`. The `Error` enum has variants for HTTP, API, JSON, URL, and config errors. API errors are parsed from the `{"error": {"error_code": ..., "error_message": ...}}` envelope.
