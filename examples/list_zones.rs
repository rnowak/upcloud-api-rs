//! List all UpCloud zones with pricing for core resources.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_zones

use upcloud_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    let zones = client.zones().list().await?;
    let price_zones = client.pricing().list_zones().await?;

    if zones.is_empty() {
        println!("No zones found.");
        return Ok(());
    }

    for z in &zones {
        let parent = z.parent_zone.as_deref().unwrap_or("(top-level)");

        println!("┌─ {} — {}", z.id, z.description);
        println!("│  Public: {}   Parent: {}", z.public, parent);

        if let Some(pz) = price_zones.iter().find(|p| p.name == z.id) {
            let core = pz.server_core.as_ref().map(|p| p.price).unwrap_or(0.0);
            let mem = pz.server_memory.as_ref().map(|p| p.price).unwrap_or(0.0);
            let ssd = pz.storage_maxiops.as_ref().map(|p| p.price).unwrap_or(0.0);
            println!("│  Pricing: core €{core:.4}/h, memory €{mem:.4}/GiB/h, SSD €{ssd:.4}/GiB/h");
        }

        println!("└─");
        println!();
    }

    println!("Total: {} zone(s)", zones.len());
    Ok(())
}
