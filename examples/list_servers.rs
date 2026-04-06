//! List all servers in your UpCloud account.
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
        println!("UUID:   {}", s.uuid);
        println!("Title:  {}", s.title);
        println!("State:  {:?}", s.state);
        println!("Zone:   {}", s.zone);
        println!("Plan:   {}", s.plan);
        println!();
    }

    println!("Total: {} server(s)", servers.len());
    Ok(())
}

