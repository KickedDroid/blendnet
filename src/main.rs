use anyhow::Context;
use anyhow::Result;
use futures_lite::StreamExt;
use iroh_net::{
    endpoint::Incoming, endpoint::SendStream, key::SecretKey, relay::RelayMode, Endpoint,
};
use std::{collections::HashMap, path::PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};
mod lib;
mod client;
use clap::{Parser, Subcommand};
use client::run_client;
use iroh_net::relay::RelayUrl;
use std::net::SocketAddr;
mod server;
use server::handle_connection;
const ALPN: &[u8] = b"n0/blendnet/0";
const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB chunks
const FILE_DELIMITER: &[u8] = b"__IROH_FILE_DELIMITER__";

type Clients = Arc<Mutex<HashMap<iroh_net::NodeId, iroh_net::endpoint::SendStream>>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Server {
        #[clap(long)]
        input: PathBuf,
        #[clap(long)]
        output: PathBuf,
    },
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
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Command::Server { input, output } => run_server(input, output).await,
        Command::Client {
            node_id,
            addrs,
            relay_url,
        } => run_client(node_id, addrs, relay_url).await,
    }
}

async fn run_server(input: PathBuf, output: PathBuf) -> Result<()> {
    println!("\nBlender Render Server\n");

    let secret_key = SecretKey::generate();
    println!("Server secret key: {secret_key}");

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![ALPN.to_vec()])
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

    println!("\nTo connect a client for rendering, run:");
    println!("cargo run -- client --node-id {server_id} --addrs \"{local_addrs}\" --relay-url {relay_url}\n");

    while let Some(incoming) = endpoint.accept().await {
        let input = input.clone();
        let output = output.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(incoming, input, output).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}
