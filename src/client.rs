use anyhow::Context;
use clap::Parser;
use iroh_net::relay::RelayUrl;
use iroh_net::{key::SecretKey, relay::RelayMode, Endpoint, NodeAddr};
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, BufReader};

const CHAT_ALPN: &[u8] = b"n0/blendnet/0";

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    node_id: iroh_net::NodeId,
    #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
    addrs: Vec<SocketAddr>,
    #[clap(long)]
    relay_url: RelayUrl,
}

pub async fn run_client(
    server_node_id: iroh_net::NodeId,
    server_addrs: Vec<SocketAddr>,
    server_relay_url: RelayUrl,
) -> anyhow::Result<()> {
    println!("\nChat Client\n");
    let secret_key = SecretKey::generate();
    println!("Client secret key: {secret_key}");

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![CHAT_ALPN.to_vec()])
        .relay_mode(RelayMode::Default)
        .bind(0)
        .await?;

    let client_id = endpoint.node_id();
    println!("Client node id: {client_id}");

    let addr = NodeAddr::from_parts(server_node_id, Some(server_relay_url), server_addrs);
    let conn = endpoint.connect(addr, CHAT_ALPN).await?;
    println!("Connected to chat server");

    let (mut send, mut recv) = conn.open_bi().await?;

    // Spawn a task to handle incoming messages
    tokio::spawn(async move {
        loop {
            match recv.read_to_end(1024).await {
                Ok(message) => {
                    let message = String::from_utf8(message).unwrap();
                    println!("{}", message);
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    // Handle user input
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    while let Some(line) = stdin.next_line().await? {
        send.write_all(line.as_bytes()).await?;
    }

    Ok(())
}
