use anyhow::Context;
use futures_lite::StreamExt;
use iroh_net::{
    endpoint::Incoming, endpoint::SendStream, key::SecretKey, relay::RelayMode, Endpoint,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};
mod client;
use clap::{Parser, Subcommand};
use client::run_client;
use iroh_net::relay::RelayUrl;
use std::net::SocketAddr;

const CHAT_ALPN: &[u8] = b"n0/blendnet/0";

type Clients = Arc<Mutex<HashMap<iroh_net::NodeId, iroh_net::endpoint::SendStream>>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Server,
    Client {
        #[clap(long)]
        node_id: iroh_net::NodeId,
        #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
        addrs: Vec<SocketAddr>,
        #[clap(long)]
        relay_url: RelayUrl,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Server => run_server().await,
        Command::Client {
            node_id,
            addrs,
            relay_url,
        } => run_client(node_id, addrs, relay_url).await,
    }
}

async fn run_server() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    println!("\nBlendnet Server\n");

    let secret_key = SecretKey::generate();
    println!("Server secret key: {secret_key}");

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![CHAT_ALPN.to_vec()])
        .relay_mode(RelayMode::Default)
        .bind(0)
        .await?;

    let server_id = endpoint.node_id();
    println!("Server node id: {server_id}");
    println!("Server listening addresses:");

    let local_addrs = endpoint
        .direct_addresses()
        .next()
        .await
        .context("no endpoints")?
        .into_iter()
        .map(|endpoint| {
            let addr = endpoint.addr.to_string();
            println!("\t{addr}");
            addr
        })
        .collect::<Vec<_>>()
        .join(" ");

    let relay_url = endpoint
        .home_relay()
        .expect("should be connected to a relay server");
    println!("Server relay url: {relay_url}");

    println!("\nTo connect, run the chat client with:");
    println!(
        "\tblendnet client --node-id {server_id} --addrs \"{local_addrs}\" --relay-url {relay_url}\n"
    );

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    while let Some(incoming) = endpoint.accept().await {
        let clients = clients.clone();
        tokio::spawn(async move { handle_connection(incoming, clients).await });
    }

    Ok(())
}

async fn handle_connection(incoming: Incoming, clients: Clients) -> anyhow::Result<()> {
    let connecting = match incoming.accept() {
        Ok(connecting) => connecting,
        Err(err) => {
            warn!("incoming connection failed: {err:#}");
            return Ok(());
        }
    };

    let conn = connecting.await?;
    let node_id = iroh_net::endpoint::get_remote_node_id(&conn)?;
    println!("New connection from {node_id}");

    let (mut send, mut recv) = conn.accept_bi().await?;

    // Add client to the list
    clients.lock().await.insert(node_id, send);

    // Handle incoming messages
    while let Ok(message) = recv.read_to_end(1024).await {
        let message = String::from_utf8(message)?;
        let broadcast_message = format!("{}: {}", node_id, message);

        // Broadcast message to all clients
        for (client_id, client_send) in clients.lock().await.iter_mut() {
            if *client_id != node_id {
                if let Err(e) = client_send.write_all(broadcast_message.as_bytes()).await {
                    warn!("Failed to send message to {}: {}", client_id, e);
                }
            }
        }
    }

    // Remove client when disconnected
    clients.lock().await.remove(&node_id);
    println!("Client {} disconnected", node_id);

    Ok(())
}
