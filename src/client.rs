use anyhow::{ Result};
use blendnet::execute_render;
use iroh_net::relay::RelayUrl;
use iroh_net::{key::SecretKey, relay::RelayMode, Endpoint, NodeAddr};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use crate::{ ALPN, CHUNK_SIZE, FILE_DELIMITER};
use blendnet::{RenderTask, RenderEngine};
use walkdir::WalkDir;


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

    //Kick off by sending a message first
    send.write_all(b"Sup").await.expect("Failed to Kick off Bi-directional Stream");

    // Receive and save Blender file
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_bytes = 0;
    let blend_file_path = PathBuf::from(format!("received_blend_file_{}.blend", total_bytes));
    let mut file = File::create(&blend_file_path).await?;

    loop {
        match recv.read(&mut buffer).await {
            Ok(None) => break, // End of stream
            Ok(Some(n)) => {
                file.write_all(&buffer[..n]).await?;
                total_bytes += n;
                println!("Received {} bytes. Total: {} bytes", n, total_bytes);
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }

    println!("Blender file received and saved. Total bytes: {}", total_bytes);

    // Create RenderTask
    let render_task = RenderTask {
        id: "Task".to_string(),
        blend_file: blend_file_path.clone(),
        start_frame: 0,
        end_frame: 1,
        render_engine: RenderEngine::Cycles,
        threads: 4,
    };

    println!("Created RenderTask: {:?}\n", render_task);

    let _res = execute_render(&render_task).expect("failed To execute Render");
    
    for entry in WalkDir::new("./output").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            println!("Sending file: {}", path.display());
    
            // Send file path (relative to output directory)
            let relative_path = path.strip_prefix("./output").unwrap_or(path);
            let path_str = relative_path.to_str().unwrap_or("unknown");
            send.write_all(&(path_str.len() as u32).to_le_bytes()).await?;
            send.write_all(path_str.as_bytes()).await?;
    
            // Send file content
            let mut file = File::open(path).await?;
            let metadata = file.metadata().await?;
            let file_size = metadata.len();
            
            send.write_all(&file_size.to_le_bytes()).await?;
    
            let mut buffer = vec![0u8; CHUNK_SIZE];
            let mut total_sent = 0;
    
            loop {
                let bytes_read = file.read(&mut buffer).await?;
                if bytes_read == 0 {
                    break;
                }
                send.write_all(&buffer[..bytes_read]).await?;
                total_sent += bytes_read;
                println!("Sent {} / {} bytes of {}", total_sent, file_size, path.display());
            }
        }
    }

    println!("Rendering complete!");
    

    send.write_all(&0u32.to_le_bytes()).await?;

    send.write_all(FILE_DELIMITER).await?;
    send.flush().await?;

    println!("All render results sent.");

    // Keep the connection open for a while to ensure the server receives the data
    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}