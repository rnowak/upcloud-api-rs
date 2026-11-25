//! List all dedicated hosts in your UpCloud account.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_hosts

use upcloud_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    let hosts = client.hosts().list().await?;

    if hosts.is_empty() {
        println!("No dedicated hosts found.");
        return Ok(());
    }

    for host in &hosts {
        println!("┌─ Host {}", host.id);
        println!("│  Description: {}", host.description);
        println!("│  Zone:        {}", host.zone);
        println!(
            "│  Windows:     {}",
            if host.windows_enabled {
                "enabled"
            } else {
                "disabled"
            }
        );

        if host.stats.stat.is_empty() {
            println!("│  Statistics:  (none)");
        } else {
            println!("│  Statistics:");
            for stat in &host.stats.stat {
                let unit = stat
                    .unit
                    .as_deref()
                    .map(|unit| format!(" {unit}"))
                    .unwrap_or_default();
                println!("│    {}: {}{}", stat.name, stat.value, unit);
            }
        }

        println!("└─");
        println!();
    }

    println!("Total: {} dedicated host(s)", hosts.len());
    Ok(())
}
