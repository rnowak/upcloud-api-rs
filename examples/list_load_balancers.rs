//! List load balancer services with their frontends, backends, and members.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_load_balancers

use upcloud_api::{Client, types::common::PageParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    let services = client.load_balancer().list(PageParams::new()).await?;

    if services.is_empty() {
        println!("No load balancer services found.");
        return Ok(());
    }

    for svc in &services {
        println!("┌─ {} ({:?})", svc.name, svc.state);
        println!("│  UUID: {}", svc.uuid);
        println!("│  Zone: {}", svc.zone);
        println!("│  Plan: {}", svc.plan);
        println!("│  DNS:  {}", svc.dns_name.as_deref().unwrap_or("—"));

        // Frontends
        let frontends = client.load_balancer().list_frontends(&svc.uuid).await?;
        if frontends.is_empty() {
            println!("│  Frontends: (none)");
        } else {
            println!("│  Frontends:");
            for fe in &frontends {
                println!(
                    "│    - {} (mode={:?}, port={}, default_backend={})",
                    fe.name, fe.mode, fe.port, fe.default_backend
                );
            }
        }

        // Backends
        let backends = client.load_balancer().list_backends(&svc.uuid).await?;
        if backends.is_empty() {
            println!("│  Backends: (none)");
        } else {
            println!("│  Backends:");
            for be in &backends {
                let resolver = be.resolver.as_deref().unwrap_or("—");
                let members = client
                    .load_balancer()
                    .list_backend_members(&svc.uuid, &be.name)
                    .await?;
                println!("│    - {} (resolver={}, {} member(s))", be.name, resolver, members.len());
                for m in &members {
                    let ip = if m.ip.is_empty() { "(dynamic)" } else { &m.ip };
                    println!(
                        "│        {} [{:?}] {}:{} weight={} enabled={}",
                        m.name,
                        m.member_type,
                        ip,
                        m.port,
                        m.weight,
                        m.enabled
                    );
                }
            }
        }

        println!("└─");
        println!();
    }

    println!("Total: {} service(s)", services.len());
    Ok(())
}
