use std::{error::Error, path::PathBuf, process::Command, str::FromStr};
mod lib;
mod client;
use lib::*;


fn test() {
    let rendertask = RenderTask {
        id: "1".to_string(),
        blend_file: PathBuf::from_str("overwerk.blend").unwrap(),
        start_frame: 0,
        end_frame: 1,
        render_engine: RenderEngine::Cycles,
        threads: 1,
    };

    execute_render(&rendertask).unwrap();
    
}

use anyhow::Context;
use futures_lite::StreamExt;
use iroh_net::{endpoint::{Connection, SendStream}, key::SecretKey, relay::RelayMode, Endpoint};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::{debug, info};

const RENDER_ALPN: &[u8] = b"n0/iroh/render/0";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("\nRender Server Starting\n");
    let secret_key = SecretKey::generate();
    println!("Secret key: {secret_key}");

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![RENDER_ALPN.to_vec()])
        .relay_mode(RelayMode::Default)
        .bind(0)
        .await?;

    let me = endpoint.node_id();
    println!("Node ID: {me}");
    println!("Listening addresses:");

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
    println!("Relay server URL: {relay_url}");

    while let Some(mut conn) = endpoint.accept().await {
        let alpn = conn.alpn().await?;
        let conn = conn.await?;
        let node_id = iroh_net::endpoint::get_remote_node_id(&conn)?;
        info!(
            "New connection from {node_id} with ALPN {} (from {})",
            String::from_utf8_lossy(&alpn),
            conn.remote_address()
        );

        tokio::spawn(async move {
            handle_connection(conn).await
        });
    }

    Ok(())
}

async fn handle_connection(conn: Connection) -> anyhow::Result<()> {
    let (mut send, mut recv) = conn.accept_bi().await?;
    debug!("Accepted bi-directional stream, sending Blend file...");
    
    let blend_file_path = PathBuf::from("path/to/your/blend/file.blend");
    send_file(&mut send, &blend_file_path).await?;

    let result = recv.read_to_end(1000).await?;
    println!("Render result: {}", String::from_utf8_lossy(&result));

    Ok(())
}

async fn send_file(send: &mut SendStream, file_path: &PathBuf) -> anyhow::Result<()> {
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    
    send.write_all(&buffer).await?;
    send.finish().await?;
    
    Ok(())
}