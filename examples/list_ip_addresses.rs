//! List all IP addresses with their server attachments and details.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_ip_addresses

use std::collections::HashMap;

use upcloud_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    let servers = client.servers().list(&Default::default()).await?;
    let server_names: HashMap<&str, &str> = servers
        .iter()
        .map(|s| (s.uuid.as_str(), s.title.as_str()))
        .collect();

    let addresses = client.ip_addresses().list().await?;

    if addresses.is_empty() {
        println!("No IP addresses found.");
        return Ok(());
    }

    // Group by server for a cleaner view.
    let mut by_server: HashMap<Option<&str>, Vec<&upcloud_api::ip_address::IpAddress>> =
        HashMap::new();
    for ip in &addresses {
        by_server
            .entry(ip.server.as_deref())
            .or_default()
            .push(ip);
    }

    // Print unattached IPs first, then grouped by server.
    if let Some(unattached) = by_server.remove(&None) {
        println!("┌─ Unattached ({} address(es))", unattached.len());
        for ip in &unattached {
            print_ip(ip);
        }
        println!("└─");
        println!();
    }

    let mut server_entries: Vec<_> = by_server.into_iter().collect();
    server_entries.sort_by_key(|(uuid, _)| *uuid);

    for (uuid, ips) in &server_entries {
        let uuid = uuid.unwrap_or("?");
        let name = server_names.get(uuid).copied().unwrap_or("(unknown)");
        println!("┌─ {} ({}) — {} address(es)", name, uuid, ips.len());
        for ip in ips {
            print_ip(ip);
        }
        println!("└─");
        println!();
    }

    println!("Total: {} IP address(es)", addresses.len());
    Ok(())
}

fn print_ip(ip: &upcloud_api::ip_address::IpAddress) {
    let floating = ip.floating.unwrap_or(false);
    let ptr = ip.ptr_record.as_deref().unwrap_or("—");
    let zone = ip.zone.as_deref().unwrap_or("—");

    println!(
        "│  {:?} {:?} {}{}",
        ip.family,
        ip.access,
        ip.address,
        if floating { " (floating)" } else { "" },
    );
    println!("│    PTR: {ptr}  Zone: {zone}");

    if let Some(mac) = &ip.mac {
        println!("│    MAC: {mac}");
    }
    if ip.part_of_plan.unwrap_or(false) {
        println!("│    Part of plan: yes");
    }
    if let Some(policy) = &ip.release_policy {
        println!("│    Release policy: {policy:?}");
    }
}
