//! List Kubernetes clusters, node groups, and available versions.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example list_kubernetes

use upcloud_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;

    // ── Available versions ──

    let versions = client.kubernetes().list_versions().await?;
    let version_list: Vec<&str> = versions.iter().map(|v| v.id.as_str()).collect();
    println!("Available Kubernetes versions: {}", version_list.join(", "));
    println!();

    // ── Clusters ──

    let clusters = client.kubernetes().list().await?;

    if clusters.is_empty() {
        println!("No Kubernetes clusters found.");
        return Ok(());
    }

    for c in &clusters {
        let version = c.version.as_deref().unwrap_or("unknown");
        let plan = c.plan.as_deref().unwrap_or("—");
        let network_cidr = c.network_cidr.as_deref().unwrap_or("—");

        println!("┌─ {} ({:?})", c.name, c.state);
        println!("│  UUID:    {}", c.uuid);
        println!("│  Zone:    {}", c.zone);
        println!("│  Version: {}", version);
        println!("│  Plan:    {}", plan);
        println!("│  CIDR:    {}", network_cidr);

        let node_groups = client.kubernetes().list_node_groups(&c.uuid).await?;

        if node_groups.is_empty() {
            println!("│  Node groups: (none)");
        } else {
            println!("│  Node groups:");
            for ng in &node_groups {
                let state = ng
                    .state
                    .as_ref()
                    .map(|s| format!("{s:?}"))
                    .unwrap_or_else(|| "—".into());
                let node_count = ng.nodes.as_ref().map(|n| n.len()).unwrap_or(0);
                println!(
                    "│    - {} ({} × {}, state={}, {} node(s))",
                    ng.name, ng.count, ng.plan, state, node_count
                );
            }
        }

        match client
            .kubernetes()
            .get_cluster_available_upgrades(&c.uuid)
            .await
        {
            Ok(upgrades) if !upgrades.versions.is_empty() => {
                println!("│  Upgrades: {}", upgrades.versions.join(", "));
            }
            _ => {}
        }

        println!("└─");
        println!();
    }

    println!("Total: {} cluster(s)", clusters.len());
    Ok(())
}
