# upcloud-api-rs

An idiomatic Rust SDK for the [UpCloud API v1.3](https://developers.upcloud.com/1.3/).

Built on **tokio** + **reqwest** + **serde**, with proper Rust types for every resource, request, and response.

## Features

- Full coverage of all 22 UpCloud API resource groups
- Domain-scoped handler pattern: `client.servers().list(&Default::default())`, `client.managed_database().create(req)`
- Short, module-scoped type names: `load_balancer::Backend`, `managed_database::User`
- Typed query parameters for filtering, ordering, and pagination on list endpoints
- Bearer token and HTTP Basic authentication
- Strongly-typed request/response structs
- Async/await throughout (`tokio`)
- TLS via `rustls` (no OpenSSL dependency)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
upcloud-api-rs = { path = "../upcloud-api-rs" }
tokio = { version = "1", features = ["full"] }
```

## Authentication

```rust
use upcloud_api_rs::Client;

// Bearer token (recommended — create tokens at https://hub.upcloud.com)
let client = Client::with_bearer_token("ucat_...")?;

// HTTP Basic auth (legacy)
let client = Client::with_basic_auth("username", "password")?;
```

### Advanced configuration

```rust
use std::time::Duration;
use upcloud_api_rs::{Client, ClientBuilder, AuthMode};

let client = Client::builder()
    .bearer_token("ucat_...")
    .base_url("https://api.upcloud.com")
    .timeout(Duration::from_secs(60))
    .build()?;
```

## Quick Start

```rust
use upcloud_api_rs::Client;

#[tokio::main]
async fn main() -> Result<(), upcloud_api_rs::Error> {
    let client = Client::with_bearer_token("ucat_...")?;

    // List zones
    let zones = client.zones().list().await?;
    for zone in &zones {
        println!("{}: {}", zone.id, zone.description);
    }

    // List servers
    let servers = client.servers().list(&Default::default()).await?;
    for server in &servers {
        println!("{} — {} ({:?})", server.uuid, server.title, server.state);
    }

    Ok(())
}
```

## API Coverage

### Account & Identity
```rust
let account = client.account().get().await?;
let subaccounts = client.account().list().await?;
let tokens = client.account().list_tokens().await?;
let token = client.account().create_token(CreateTokenRequest { name: "ci".into(), ..Default::default() }).await?;
client.account().delete_token(&token.id).await?;
```

### Servers
```rust
use upcloud_api_rs::server::*;

// List / get
let servers = client.servers().list(&Default::default()).await?;
let server = client.servers().get("uuid").await?;

// Filtered / paginated list
let filtered = client.servers().list(
    &ListParams::new()
        .state(State::Started)
        .label("env", "prod")
        .order_by(OrderBy::Title, upcloud_api_rs::types::common::SortOrder::Asc)
        .limit(25)
).await?;

// Create
let req = CreateRequest {
    title: "my-server".into(),
    hostname: "my-server.example.com".into(),
    zone: "de-fra1".into(),
    ..Default::default()
};
let server = client.servers().create(req).await?;

// Lifecycle
client.servers().start("uuid").await?;
client.servers().stop("uuid", StopRequest { stop_type: StopType::Soft, ..Default::default() }).await?;
client.servers().restart("uuid", RestartRequest::default()).await?;
client.servers().delete("uuid").await?;

// Server Groups
let groups = client.servers().list_groups().await?;
```

### Storages
```rust
use upcloud_api_rs::storage::*;

let storages = client.storages().list(&Default::default()).await?;
let storage = client.storages().get("uuid").await?;
let created = client.storages().create(CreateRequest { size: 20, title: "disk".into(), zone: "de-fra1".into(), ..Default::default() }).await?;
let backup = client.storages().create_backup("uuid", CreateBackupRequest { title: "snap".into() }).await?;
client.storages().delete("uuid").await?;
```

### IP Addresses
```rust
use upcloud_api_rs::ip_address::*;

let ips = client.ip_addresses().list().await?;
let ip = client.ip_addresses().assign(AssignRequest {
    server: "server-uuid".into(),
    family: Family::IPv4,
    ..Default::default()
}).await?;
client.ip_addresses().release("1.2.3.4").await?;
```

### Networks
```rust
let networks = client.networks().list().await?;
let routers = client.networks().list_routers().await?;
let peerings = client.networks().list_peerings().await?;
```

### Managed Databases
```rust
use upcloud_api_rs::managed_database::*;

let services = client.managed_database().list().await?;
let plans = client.managed_database().list_plans().await?;
let db = client.managed_database().create(CreateRequest {
    title: "my-pg".into(),
    plan: "2x2xCPU-4GB-50GB-PG17".into(),
    zone: "de-fra1".into(),
    db_type: "pg".into(),
    ..Default::default()
}).await?;
```

### Load Balancer
```rust
use upcloud_api_rs::load_balancer::*;

let lbs = client.load_balancer().list(Default::default()).await?;
let plans = client.load_balancer().list_plans(Default::default()).await?;
let lb = client.load_balancer().create(CreateRequest {
    name: "my-lb".into(),
    plan: "production-2".into(),
    zone: "de-fra1".into(),
    ..Default::default()
}).await?;
let backends = client.load_balancer().list_backends(&lb.uuid).await?;
let frontends = client.load_balancer().list_frontends(&lb.uuid).await?;
```

### Kubernetes
```rust
use upcloud_api_rs::kubernetes::*;

let clusters = client.kubernetes().list().await?;
let plans = client.kubernetes().list_plans().await?;
let cluster = client.kubernetes().create(CreateClusterRequest {
    name: "my-cluster".into(),
    plan: "K8S-2xCPU-4GB".into(),
    zone: "de-fra1".into(),
    ..Default::default()
}).await?;
let kubeconfig = client.kubernetes().get_kubeconfig(&cluster.uuid).await?;
```

### Object Storage
```rust
use upcloud_api_rs::object_storage::*;

let services = client.object_storage().list(Default::default()).await?;
let regions = client.object_storage().list_regions().await?;
```

### Network Gateway
```rust
use upcloud_api_rs::gateway::*;

let gateways = client.gateway().list(Default::default()).await?;
let plans = client.gateway().list_plans().await?;
```

### File Storage
```rust
use upcloud_api_rs::file_storage::*;

let storages = client.file_storage().list(Default::default()).await?;
let plans = client.file_storage().list_plans().await?;
```

### Tags
```rust
let tags = client.tags().list().await?;
client.tags().tag_server("server-uuid", "production,web").await?;
client.tags().untag_server("server-uuid", "staging").await?;
```

### Firewall Rules
```rust
use upcloud_api_rs::server::firewall::*;

let rules = client.servers().list_firewall_rules("server-uuid").await?;
let rule = client.servers().create_firewall_rule("server-uuid", CreateRuleRequest {
    direction: Direction::In,
    action: Action::Accept,
    position: 1,
    ..Default::default()
}).await?;
```

### Zones, Pricing, Plans
```rust
let zones = client.zones().list().await?;
let prices = client.pricing().list_zones().await?;
let plans = client.plans().list().await?;
let server_configs = client.plans().list_server_configurations().await?;
let timezones = client.timezones().list().await?;
```

### Audit Log
```rust
use upcloud_api_rs::audit_log::AuditListParams;

let page = client.audit_log().list(AuditListParams {
    limit: Some(100),
    ..Default::default()
}).await?;
println!("{} total events, showing {}", page.total_count, page.events.len());
```

### Hosts (Dedicated)
```rust
let hosts = client.hosts().list().await?;
let host = client.hosts().get(12345).await?;
```

### Permissions
```rust
use upcloud_api_rs::permission::*;

let perms = client.permissions().list().await?;
client.permissions().grant(GrantRequest { target_identifier: "uuid".into(), ..Default::default() }).await?;
```

### Partners
```rust
let accounts = client.partners().list().await?;
```

## Error Handling

```rust
use upcloud_api_rs::{Client, Error};

match client.servers().get("nonexistent").await {
    Ok(server) => println!("Found: {}", server.title),
    Err(Error::Api(api_err)) => {
        eprintln!("UpCloud error {}: {} ({})", api_err.status, api_err.error_message, api_err.error_code);
    }
    Err(Error::Http(e)) => eprintln!("HTTP error: {e}"),
    Err(e) => eprintln!("Other error: {e}"),
}
```

## Module Structure

```
upcloud_api_rs/
├── src/
│   ├── lib.rs                     # Crate root, prelude
│   ├── client.rs                  # Client, ClientBuilder, handler factories
│   ├── config.rs                  # ClientConfig, AuthMode
│   ├── error.rs                   # Error, ApiError
│   ├── types/common.rs            # Shared helpers (YesNo, StringNumber, Label, …)
│   │
│   ├── server/                    # Servers, server groups, firewall rules
│   ├── storage/                   # Block storages, backups, imports
│   ├── network/                   # Networks, routers, peerings, interfaces
│   ├── ip_address/                # IP address management
│   ├── load_balancer/             # Load balancers + backends/frontends
│   ├── managed_database/          # Managed databases
│   ├── object_storage/            # Object storage (S3-compatible)
│   ├── kubernetes/                # Kubernetes clusters + node groups
│   ├── gateway/                   # Network gateways
│   ├── file_storage/              # File storage + shares
│   ├── account/                   # Account, subaccounts, API tokens
│   ├── tag/                       # Tags
│   ├── host/                      # Dedicated hosts
│   ├── plan/                      # Server plans + sizes
│   ├── pricing/                   # Zone pricing
│   ├── zone/                      # Data center zones
│   ├── timezone/                  # Timezone identifiers
│   ├── audit_log/                 # Audit events
│   ├── permission/                # Role/permission management
│   └── partner/                   # Partner API
```

Each domain module exposes:
- **Types** — short, module-scoped names (e.g. `load_balancer::Backend`)
- **Handler** — `{Domain}Handler<'a>` struct with domain methods
- **Factory** — `client.{domain}()` returns the handler

## License

MIT
