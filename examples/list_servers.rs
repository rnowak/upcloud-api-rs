//! List all servers in your UpCloud account with resource details.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_servers

use upcloud_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    let servers = client.servers().list(&Default::default()).await?;

    if servers.is_empty() {
        println!("No servers found.");
        return Ok(());
    }

    for s in &servers {
        let tags = s
            .tags
            .as_ref()
            .map(|t| {
                t.tag
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| "(none)".into());

        println!("┌─ {} ({})", s.title, s.uuid);
        println!("│  Hostname: {}", s.hostname);
        println!("│  State:    {:?}", s.state);
        println!("│  Zone:     {}", s.zone);
        println!("│  Plan:     {}", s.plan);
        println!("│  CPUs:     {}", s.core_number);
        println!("│  RAM:      {}", format_mib(s.memory_amount));
        println!("│  Tags:     {}", tags);
        println!("└─");
        println!();
    }

    println!("Total: {} server(s)", servers.len());
    Ok(())
}

fn format_mib(mib: u64) -> String {
    if mib >= 1024 {
        format!("{} GiB", mib / 1024)
    } else {
        format!("{mib} MiB")
    }
}
