//! List private storages and show which servers they are attached to.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_storages

use std::collections::HashMap;

use upcloud_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    // Build a server UUID → title map so we can resolve attachment names.
    let servers = client.servers().list(&Default::default()).await?;
    let server_names: HashMap<&str, &str> = servers
        .iter()
        .map(|s| (s.uuid.as_str(), s.title.as_str()))
        .collect();

    let storages = client.storages().list_private().await?;

    if storages.is_empty() {
        println!("No private storages found.");
        return Ok(());
    }

    for s in &storages {
        let zone = s.zone.as_deref().unwrap_or("unknown");
        let tier = s.tier.as_ref().map(|t| format!("{t:?}")).unwrap_or_else(|| "—".into());
        let encrypted = s.encrypted.map(|e| if e { "yes" } else { "no" }).unwrap_or("—");

        println!("┌─ {} ({})", s.title, s.uuid);
        println!("│  Type:      {:?}", s.storage_type);
        println!("│  State:     {:?}", s.state);
        println!("│  Size:      {} GiB", s.size);
        println!("│  Tier:      {}", tier);
        println!("│  Zone:      {}", zone);
        println!("│  Encrypted: {}", encrypted);

        // Fetch full details to find server attachment.
        match client.storages().get(&s.uuid).await {
            Ok(details) => {
                let attached: Vec<&str> = details
                    .servers
                    .as_ref()
                    .map(|ss| {
                        ss.server
                            .iter()
                            .map(|uuid| {
                                server_names.get(uuid.as_str()).copied().unwrap_or(uuid.as_str())
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                if attached.is_empty() {
                    println!("│  Attached:  (detached)");
                } else {
                    println!("│  Attached:  {}", attached.join(", "));
                }
            }
            Err(e) => {
                println!("│  Attached:  (error: {e})");
            }
        }

        println!("└─");
        println!();
    }

    println!("Total: {} private storage(s)", storages.len());
    Ok(())
}
