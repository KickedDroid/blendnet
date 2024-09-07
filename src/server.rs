use std::path::PathBuf;

use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tracing::{info, warn};

use crate::{CHUNK_SIZE, FILE_DELIMITER};

use anyhow::Result;






pub async fn handle_connection(incoming: iroh_net::endpoint::Incoming, input: PathBuf, output: PathBuf) -> Result<()> {
    let connecting = match incoming.accept() {
        Ok(connecting) => connecting,
        Err(err) => {
            println!("incoming connection failed: {err:#}");
            return Ok(());
        }
    };

    let conn = connecting.await?;
    let node_id = iroh_net::endpoint::get_remote_node_id(&conn)?;
    println!("New connection from {node_id}");

    let (mut send, mut recv) = conn.accept_bi().await.expect("Faield");
    println!("Connected??");
    // Send Blender file to client
    let mut file = File::open(&input).await?;
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_bytes = 0;

    loop {
        println!("PLEAWEE");
        match file.read(&mut buffer).await {
            Ok(0) => break, // End of file
            Ok(n) => {
                send.write_all(&buffer[..n]).await?;
                total_bytes += n;
                println!("Sent {} bytes. Total: {} bytes", n, total_bytes);
            }
            Err(e) => {
                println!("Error reading from file: {}", e);
                break;
            }
        }
    }
    send.write_all(FILE_DELIMITER).await?;
    send.finish()?;
    println!("Blender file sent. Total bytes: {}", total_bytes);

    // Receive rendered result from client
    let mut output_file = File::create(&output).await?;
    let mut total_bytes = 0;

    loop {
        match recv.read(&mut buffer).await {
            Ok(None) => break, // End of stream
            Ok(Some(n)) => {
                output_file.write_all(&buffer[..n]).await?;
                total_bytes += n;
                println!("Received {} bytes. Total: {} bytes", n, total_bytes);
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }

    println!("Render result received. Total bytes: {}", total_bytes);
    Ok(())
}