# upcloud-api-rs

An idiomatic Rust SDK for the [UpCloud API v1.3](https://developers.upcloud.com/1.3/).

Built on **tokio** + **reqwest** + **serde**, with proper Rust types for every resource, request, and response.

## Features

- Full coverage of all 25 UpCloud API resource groups
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
    let zones = client.list_zones().await?;
    for zone in &zones {
        println!("{}: {}", zone.id, zone.description);
    }

    // List servers
    let servers = client.list_servers().await?;
    for server in &servers {
        println!("{} — {} ({:?})", server.uuid, server.title, server.state);
    }

    Ok(())
}
```

## API Coverage

### Account & Identity
```rust
let account = client.get_account().await?;
let subaccounts = client.list_subaccounts().await?;
let tokens = client.list_api_tokens().await?;
let token = client.create_api_token(CreateTokenRequest { name: "ci".into(), ..Default::default() }).await?;
client.delete_api_token(&token.id).await?;
```

### Servers
```rust
use upcloud_api_rs::api::servers::*;

// List / get
let servers = client.list_servers().await?;
let server = client.get_server("uuid").await?;

// Create
let req = CreateServerRequest {
    title: "my-server".into(),
    hostname: "my-server.example.com".into(),
    zone: "de-fra1".into(),
    ..Default::default()
};
let server = client.create_server(req).await?;

// Lifecycle
client.start_server("uuid").await?;
client.stop_server("uuid", StopServerRequest { stop_type: StopType::Soft, ..Default::default() }).await?;
client.restart_server("uuid", RestartServerRequest::default()).await?;
client.delete_server("uuid").await?;

// Server Groups
let groups = client.list_server_groups().await?;
```

### Storages
```rust
use upcloud_api_rs::api::storages::*;

let storages = client.list_storages().await?;
let storage = client.get_storage("uuid").await?;
let created = client.create_storage(CreateStorageRequest { size: 20, title: "disk".into(), zone: "de-fra1".into(), ..Default::default() }).await?;
let backup = client.create_backup("uuid", CreateBackupRequest { title: "snap".into() }).await?;
client.delete_storage("uuid").await?;
```

### IP Addresses
```rust
use upcloud_api_rs::api::ip_addresses::*;

let ips = client.list_ip_addresses().await?;
let ip = client.assign_ip_address(AssignIpAddressRequest {
    server: "server-uuid".into(),
    family: IpAddressFamily::IPv4,
    ..Default::default()
}).await?;
client.release_ip_address("1.2.3.4").await?;
```

### Networks
```rust
let networks = client.list_networks().await?;
let routers = client.list_routers().await?;
let peerings = client.list_network_peerings().await?;
```

### Managed Databases
```rust
use upcloud_api_rs::api::managed_database::*;

let services = client.list_managed_databases().await?;
let plans = client.list_managed_database_plans().await?;
let db = client.create_managed_database(CreateManagedDatabaseRequest {
    title: "my-pg".into(),
    plan: "2x2xCPU-4GB-50GB-PG17".into(),
    zone: "de-fra1".into(),
    db_type: "pg".into(),
    ..Default::default()
}).await?;
```

### Load Balancer
```rust
use upcloud_api_rs::api::load_balancer::*;

let lbs = client.list_load_balancers().await?;
let plans = client.list_load_balancer_plans().await?;
let lb = client.create_load_balancer(CreateLoadBalancerRequest {
    name: "my-lb".into(),
    plan: "production-2".into(),
    zone: "de-fra1".into(),
    ..Default::default()
}).await?;
let backends = client.list_load_balancer_backends(&lb.uuid).await?;
let frontends = client.list_load_balancer_frontends(&lb.uuid).await?;
```

### Kubernetes
```rust
use upcloud_api_rs::api::kubernetes::*;

let clusters = client.list_kubernetes_clusters().await?;
let plans = client.list_kubernetes_plans().await?;
let cluster = client.create_kubernetes_cluster(CreateKubernetesClusterRequest {
    name: "my-cluster".into(),
    plan: "K8S-2xCPU-4GB".into(),
    zone: "de-fra1".into(),
    ..Default::default()
}).await?;
let kubeconfig = client.get_kubernetes_kubeconfig(&cluster.uuid).await?;
```

### Managed Object Storage
```rust
use upcloud_api_rs::api::managed_object_storage::*;

let services = client.list_managed_object_storages().await?;
let regions = client.list_managed_object_storage_regions().await?;
```

### Network Gateway
```rust
use upcloud_api_rs::api::gateway::*;

let gateways = client.list_gateways().await?;
let plans = client.list_gateway_plans().await?;
```

### File Storage
```rust
use upcloud_api_rs::api::file_storage::*;

let storages = client.list_file_storages().await?;
let plans = client.list_file_storage_plans().await?;
```

### Tags
```rust
use upcloud_api_rs::api::tags::*;

let tags = client.list_tags().await?;
client.tag_server("server-uuid", &["production", "web"]).await?;
client.untag_server("server-uuid", &["staging"]).await?;
```

### Firewall Rules
```rust
use upcloud_api_rs::api::firewall::*;

let rules = client.get_server_firewall_rules("server-uuid").await?;
let rule = client.create_firewall_rule("server-uuid", CreateFirewallRuleRequest {
    direction: FirewallDirection::In,
    action: FirewallAction::Accept,
    position: 1,
    ..Default::default()
}).await?;
```

### Zones, Pricing, Plans
```rust
let zones = client.list_zones().await?;
let prices = client.list_price_zones().await?;
let plans = client.list_plans().await?;
let server_sizes = client.list_server_configurations().await?;
let timezones = client.list_timezones().await?;
```

### Audit Log
```rust
use upcloud_api_rs::api::audit_log::AuditListParams;

let page = client.list_audit_events(AuditListParams {
    limit: Some(100),
    ..Default::default()
}).await?;
println!("{} total events, showing {}", page.total_count, page.events.len());
```

### Hosts (Dedicated)
```rust
let hosts = client.list_hosts().await?;
let host = client.get_host(12345).await?;
```

### Permissions
```rust
use upcloud_api_rs::api::permissions::*;

let perms = client.list_permissions().await?;
client.grant_permission(GrantPermissionRequest { target_identifier: "uuid".into(), ..Default::default() }).await?;
```

### Partners
```rust
use upcloud_api_rs::api::partners::*;

let accounts = client.list_partner_accounts().await?;
```

## Error Handling

```rust
use upcloud_api_rs::{Client, Error};

match client.get_server("nonexistent").await {
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
│   ├── lib.rs                        # Crate root, prelude
│   ├── client.rs                     # Client, ClientBuilder
│   ├── config.rs                     # ClientConfig, AuthMode
│   ├── error.rs                      # Error, ApiError
│   ├── types/
│   │   └── common.rs                 # Shared helpers (YesNo, StringNumber, Label, …)
│   └── api/
│       ├── account.rs                # Account, subaccounts, API tokens
│       ├── audit_log.rs              # Audit events
│       ├── file_storage.rs           # File storage + shares
│       ├── firewall.rs               # Firewall rules (shared types)
│       ├── gateway.rs                # Network gateways
│       ├── hosts.rs                  # Dedicated hosts
│       ├── ip_addresses.rs           # IP address management
│       ├── kubernetes.rs             # Kubernetes clusters + node groups
│       ├── load_balancer.rs          # Load balancers + backends/frontends
│       ├── managed_database.rs       # Managed databases
│       ├── managed_object_storage.rs # Object storage (S3-compatible)
│       ├── networks.rs               # Networks, routers, peerings
│       ├── partners.rs               # Partner API
│       ├── permissions.rs            # Role/permission management
│       ├── plans.rs                  # Server plans + sizes
│       ├── pricing.rs                # Pricing + timezones
│       ├── servers.rs                # Servers, server groups
│       ├── storages.rs               # Block storages
│       ├── tags.rs                   # Tags
│       └── zones.rs                  # Zones
```

## License

MIT
