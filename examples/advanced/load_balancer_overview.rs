//! Deep overview of load balancer services — frontends, backends, members,
//! health checks, TLS configs, resolvers, networks, and nodes.
//!
//! Demonstrates the difference between the embedded data returned by
//! `list_backends` (which includes members inline) and the dedicated
//! `list_backend_members` / `get_backend_member` endpoints.
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example load_balancer_overview

use upcloud_api::{Client, types::common::PageParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set the UPCLOUD_TOKEN environment variable to your UpCloud bearer token");

    let client = Client::with_bearer_token(token)?;
    let lb = client.load_balancer();

    let services = lb.list(PageParams::new()).await?;

    if services.is_empty() {
        println!("No load balancer services found.");
        return Ok(());
    }

    for svc in &services {
        println!("╔══ {} ({:?})", svc.name, svc.state);
        println!("║  UUID:       {}", svc.uuid);
        println!("║  Zone:       {}", svc.zone);
        println!("║  Plan:       {}", svc.plan);
        println!("║  DNS:        {}", svc.dns_name.as_deref().unwrap_or("—"));
        println!(
            "║  Status:     {}",
            svc.configured_status
                .map(|s| format!("{:?}", s))
                .as_deref()
                .unwrap_or("—")
        );

        if let (Some(dow), Some(time)) = (&svc.maintenance_dow, &svc.maintenance_time) {
            println!("║  Maintenance: {dow} at {time} UTC");
        }

        // ── Networks ──────────────────────────────────────────────────────
        if svc.networks.is_empty() {
            println!("║  Networks:   (none)");
        } else {
            println!("║  Networks:");
            for net in &svc.networks {
                let family = net.family.map(|f| format!("{:?}", f));
                let family = family.as_deref().unwrap_or("—");
                let dns = net.dns_name.as_deref().unwrap_or("—");
                println!(
                    "║    ▸ {} [{:?}] family={} dns={}",
                    net.name, net.network_type, family, dns
                );
            }
        }

        // ── IP Addresses ──────────────────────────────────────────────────
        if let Some(ips) = &svc.ip_addresses {
            if !ips.is_empty() {
                println!("║  IP Addresses:");
                for ip in ips {
                    let net = ip.network_name.as_deref().unwrap_or("—");
                    println!("║    ▸ {} (network={})", ip.address, net);
                }
            }
        }

        // ── Nodes ─────────────────────────────────────────────────────────
        if let Some(nodes) = &svc.nodes {
            if !nodes.is_empty() {
                println!("║  Nodes:");
                for (i, node) in nodes.iter().enumerate() {
                    let state = node.operational_state.as_deref().unwrap_or("—");
                    println!("║    Node {} ({})", i + 1, state);
                    for net in &node.networks {
                        let ips: Vec<&str> = net
                            .ip_addresses
                            .iter()
                            .map(|ip| ip.address.as_str())
                            .collect();
                        println!(
                            "║      {} [{:?}]: {}",
                            net.name,
                            net.network_type,
                            if ips.is_empty() {
                                "(no IPs)".to_string()
                            } else {
                                ips.join(", ")
                            }
                        );
                    }
                }
            }
        }

        // ── Resolvers ─────────────────────────────────────────────────────
        if !svc.resolvers.is_empty() {
            println!("║  Resolvers:");
            for r in &svc.resolvers {
                println!(
                    "║    ▸ {} → [{}] (retries={}, timeout={}s, cache_valid={}s)",
                    r.name,
                    r.nameservers.join(", "),
                    r.retries,
                    r.timeout,
                    r.cache_valid,
                );
            }
        }

        // ── Frontends ─────────────────────────────────────────────────────
        let frontends = lb.list_frontends(&svc.uuid).await?;
        if frontends.is_empty() {
            println!("║  Frontends:  (none)");
        } else {
            println!("║");
            println!("║  Frontends:");
            for fe in &frontends {
                println!(
                    "║  ┌─ {} (mode={:?}, port={}, → backend={})",
                    fe.name, fe.mode, fe.port, fe.default_backend
                );

                if !fe.networks.is_empty() {
                    let nets: Vec<&str> = fe.networks.iter().map(|n| n.name.as_str()).collect();
                    println!("║  │  Networks: {}", nets.join(", "));
                }

                if !fe.tls_configs.is_empty() {
                    println!("║  │  TLS configs:");
                    for tls in &fe.tls_configs {
                        let cert = tls.certificate_bundle_uuid.as_deref().unwrap_or("(inline)");
                        println!("║  │    ▸ {} (cert={})", tls.name, cert);
                    }
                }

                if !fe.rules.is_empty() {
                    println!("║  │  Rules:");
                    for rule in &fe.rules {
                        let cond = rule.matching_condition.map(|c| format!("{:?}", c));
                        let cond = cond.as_deref().unwrap_or("and");
                        println!(
                            "║  │    ▸ {} (priority={}, match={}, {} matcher(s), {} action(s))",
                            rule.name,
                            rule.priority,
                            cond,
                            rule.matchers.len(),
                            rule.actions.len(),
                        );
                    }
                }

                println!("║  └─");
            }
        }

        // ── Backends (using dedicated member endpoints) ───────────────────
        let backends = lb.list_backends(&svc.uuid).await?;
        if backends.is_empty() {
            println!("║  Backends:   (none)");
        } else {
            println!("║");
            println!("║  Backends:");
            for be in &backends {
                let mode = be.mode.map(|m| format!("{:?}", m));
                let mode = mode.as_deref().unwrap_or("—");
                let resolver = be.resolver.as_deref().unwrap_or("—");

                println!("║  ┌─ {} (mode={}, resolver={})", be.name, mode, resolver);

                if let Some(hc) = &be.health_check {
                    let path = hc.http_path.as_deref().unwrap_or("—");
                    println!(
                        "║  │  Health check: type={:?}, interval={}s, timeout={}s, \
                         healthy={}, unhealthy={}, path={}",
                        hc.check_type,
                        hc.interval,
                        hc.timeout,
                        hc.healthy_threshold,
                        hc.unhealthy_threshold,
                        path,
                    );
                }

                if !be.tls_configs.is_empty() {
                    println!("║  │  TLS configs:");
                    for tls in &be.tls_configs {
                        let cert = tls.certificate_bundle_uuid.as_deref().unwrap_or("(inline)");
                        println!("║  │    ▸ {} (cert={})", tls.name, cert);
                    }
                }

                // Use the dedicated list_backend_members endpoint rather than
                // the embedded `be.members` from list_backends — this shows
                // how to use the member-specific handlers.
                let members = lb.list_backend_members(&svc.uuid, &be.name).await?;

                if members.is_empty() {
                    println!("║  │  Members: (none)");
                } else {
                    println!("║  │  Members ({}):", members.len());
                    for m in &members {
                        let ip = if m.ip.is_empty() { "(dynamic)" } else { &m.ip };
                        let backup = m
                            .backup
                            .map(|b| if b { " BACKUP" } else { "" })
                            .unwrap_or("");
                        let status = if m.enabled { "✓" } else { "✗" };

                        println!(
                            "║  │    {} {} [{:?}] {}:{} w={} max_sess={}{backup}",
                            status, m.name, m.member_type, ip, m.port, m.weight, m.max_sessions,
                        );

                        // Use get_backend_member to show per-member timestamps
                        // (demonstrating the individual GET endpoint).
                        let detail = lb.get_backend_member(&svc.uuid, &be.name, &m.name).await?;
                        if let Some(created) = &detail.created_at {
                            println!("║  │      created: {created}");
                        }
                        if let Some(updated) = &detail.updated_at {
                            println!("║  │      updated: {updated}");
                        }
                    }
                }

                println!("║  └─");
            }
        }

        // ── Labels ────────────────────────────────────────────────────────
        if let Some(labels) = &svc.labels {
            if !labels.is_empty() {
                let formatted: Vec<String> = labels
                    .iter()
                    .map(|l| format!("{}={}", l.key, l.value))
                    .collect();
                println!("║  Labels: {}", formatted.join(", "));
            }
        }

        println!("╚══");
        println!();
    }

    println!("Total: {} load balancer service(s)", services.len());
    Ok(())
}
