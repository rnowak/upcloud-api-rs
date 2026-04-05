//! Debug: print the raw JSON from GET /1.3/server
//!
//! Usage:
//!   UPCLOUD_TOKEN=ucat_... cargo run --example debug_raw_servers

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("UPCLOUD_TOKEN")
        .expect("Set UPCLOUD_TOKEN");

    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.upcloud.com/1.3/server")
        .bearer_auth(&token)
        .header("Accept", "application/json")
        .send()
        .await?;

    let text = resp.text().await?;
    let val: serde_json::Value = serde_json::from_str(&text)?;
    println!("{}", serde_json::to_string_pretty(&val)?);

    Ok(())
}
