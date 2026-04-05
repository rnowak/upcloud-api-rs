//! List all servers in your UpCloud account.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_servers

use upcloud_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    let servers = client.list_servers().await?;

    if servers.is_empty() {
        println!("No servers found.");
        return Ok(());
    }

    for s in &servers {
        let labels_str = match s.labels.as_ref() {
            None => String::from("(none)"),
            Some(l) if l.label.is_empty() => String::from("(none)"),
            Some(l) => l.label.iter().map(|l| format!("{}={}", l.key, l.value)).collect::<Vec<_>>().join(", "),
        };

        println!("UUID:   {}", s.uuid);
        println!("Title:  {}", s.title);
        println!("State:  {:?}", s.state);
        println!("Zone:   {}", s.zone);
        println!("Plan:   {}", s.plan);
        println!("Labels: {}", labels_str);
        println!();
    }

    println!("Total: {} server(s)", servers.len());
    Ok(())
}

