//! List all managed object storage services and their buckets.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_object_storages

use upcloud_sdk::{Client, types::common::PageParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    let services = client.list_mos_services(PageParams::new()).await?;

    if services.is_empty() {
        println!("No managed object storage services found.");
        return Ok(());
    }

    for svc in &services {
        let state = svc.operational_state.as_deref().unwrap_or("unknown");
        let endpoint = svc.endpoints.as_ref()
            .and_then(|e| e.iter().find(|e| e.endpoint_type == "public").or_else(|| e.first()))
            .map(|e| e.domain_name.as_str())
            .unwrap_or("(no endpoint)");

        println!("┌─ {} ({})", svc.name, svc.uuid);
        println!("│  Region:   {}", svc.region);
        println!("│  State:    {}", state);
        println!("│  Endpoint: {}", endpoint);

        match client.list_mos_buckets(&svc.uuid).await {
            Ok(buckets) if buckets.is_empty() => {
                println!("│  Buckets:  (none)");
            }
            Ok(buckets) => {
                println!("│  Buckets:");
                for b in &buckets {
                    let objects = b.total_objects.unwrap_or(0);
                    let size = b.total_size_bytes.map(format_bytes).unwrap_or_default();
                    println!("│    - {} ({} objects, {})", b.name, objects, size);
                }
            }
            Err(e) => {
                println!("│  Buckets:  (error: {e})");
            }
        }

        println!("└─");
        println!();
    }

    println!("Total: {} service(s)", services.len());
    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    const KIB: u64 = 1024;
    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}


