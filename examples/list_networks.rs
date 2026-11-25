//! List all networks and IP addresses in your UpCloud account.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_networks

use upcloud_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    // ── Networks ──

    let networks = client.networks().list().await?;

    if networks.is_empty() {
        println!("No networks found.");
    } else {
        println!("═══ Networks ═══");
        println!();

        for n in &networks {
            let subnets: Vec<String> = n
                .ip_networks
                .as_ref()
                .map(|ip| {
                    ip.ip_network
                        .iter()
                        .map(|s| format!("{} ({:?})", s.address, s.family,))
                        .collect()
                })
                .unwrap_or_default();

            let server_count = n.servers.as_ref().map(|s| s.server.len()).unwrap_or(0);

            println!("┌─ {} ({:?})", n.name, n.network_type);
            println!("│  UUID:    {}", n.uuid);
            println!("│  Zone:    {}", n.zone);
            if subnets.is_empty() {
                println!("│  Subnets: (none)");
            } else {
                for (i, s) in subnets.iter().enumerate() {
                    if i == 0 {
                        println!("│  Subnets: {s}");
                    } else {
                        println!("│           {s}");
                    }
                }
            }
            println!("│  Servers: {server_count}");
            println!("└─");
            println!();
        }

        println!("Total: {} network(s)", networks.len());
        println!();
    }

    // ── IP addresses ──

    let ips = client.ip_addresses().list().await?;

    if ips.is_empty() {
        println!("No IP addresses found.");
    } else {
        println!("═══ IP Addresses ═══");
        println!();

        for ip in &ips {
            let server = ip.server.as_deref().unwrap_or("(unattached)");
            let ptr = ip.ptr_record.as_deref().unwrap_or("—");
            let floating = ip
                .floating
                .map(|f| if f { "yes" } else { "no" })
                .unwrap_or("—");

            println!(
                "  {:>15}  {:?}/{:?}  floating={}  server={}  ptr={}",
                ip.address, ip.access, ip.family, floating, server, ptr
            );
        }

        println!();
        println!("Total: {} address(es)", ips.len());
    }

    Ok(())
}
