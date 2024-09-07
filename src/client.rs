use anyhow::Context;
use clap::Parser;
use iroh_net::relay::RelayUrl;
use iroh_net::{key::SecretKey, relay::RelayMode, Endpoint, NodeAddr};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use anyhow::Result;
use crate::{ALPN, CHUNK_SIZE, FILE_DELIMITER};


#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    node_id: iroh_net::NodeId,
    #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
    addrs: Vec<SocketAddr>,
    #[clap(long)]
    relay_url: RelayUrl,
}

pub async fn run_client(server_node_id: iroh_net::NodeId, server_addrs: Vec<SocketAddr>, server_relay_url: RelayUrl) -> Result<()> {
    println!("\nBlender Render Client\n");
    let secret_key = SecretKey::generate();
    println!("Client secret key: {secret_key}");

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![ALPN.to_vec()])
        .relay_mode(RelayMode::Default)
        .bind(0)
        .await?;

    let client_id = endpoint.node_id();
    println!("Client node id: {client_id}");

    let addr = NodeAddr::from_parts(server_node_id, Some(server_relay_url), server_addrs);
    let conn = endpoint.connect(addr, ALPN).await?;
    println!("Connected to Blender Render server");

    let (mut send, mut recv) = conn.open_bi().await?;
    send.write(b"hi").await?;
    // Receive Blender file from server
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_bytes = 0;

    loop {
        match recv.read(&mut buffer).await {
            Ok(None) => break, // End of stream
            Ok(Some(n)) => {
                total_bytes += n;
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }

    println!("Blender file received. Total bytes: {}", total_bytes);

    // Replace with execute_render
    println!("Simulating rendering process...");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Rendering complete!");

    // Sim
    let simulated_result = b"This is a simulated render result.".to_vec();
    send.write_all(&simulated_result).await?;
    send.write_all(FILE_DELIMITER).await?;
    send.finish()?;

    println!("Render result sent. Total bytes: {}", simulated_result.len());

    Ok(())
}