use std::{path::PathBuf, str::MatchIndices};

use iroh_net::endpoint::RecvStream;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{info, warn};

use crate::{lib::RenderResult, CHUNK_SIZE, FILE_DELIMITER};

use anyhow::{ Result};

pub async fn handle_connection(
    incoming: iroh_net::endpoint::Incoming,
    input: PathBuf,
    output: PathBuf,
) -> Result<()> {
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

    // Read and discard the initial "Sup" message
    let mut initial_buffer = [0u8; 3];
    recv.read_exact(&mut initial_buffer).await?;
    let initial_message = std::str::from_utf8(&initial_buffer)?;
    println!("Received initial message: {}", initial_message);

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

    match receive_render_results(&mut recv).await {
        Ok(_) => {
            println!("Got Render Results");

            Ok(())
        }
        Err(e) => {
            println!("Failed to send Renders");
            Ok(())
        }
    }
}

async fn receive_render_results(recv: &mut RecvStream) -> Result<()> {
    
    let mut total_bytes = 0;
    loop {
        // Read file path length
        let mut path_len_bytes = [0u8; 4];
        if recv.read_exact(&mut path_len_bytes).await.is_err() {
            break; // End of stream
        }
        let path_len = u32::from_le_bytes(path_len_bytes);

        if path_len == 0 {
            println!("Received end-of-files marker");
            break; // No more files
        }

        // Read file path
        let mut path_bytes = vec![0u8; path_len as usize];
        recv.read_exact(&mut path_bytes).await?;
        let file_path = String::from_utf8(path_bytes)?;
        let output_path = PathBuf::from("final").join(file_path);

        // Ensure the directory exists
        if let Some(parent) = output_path.parent() {
            create_dir_all(parent).await?;
        }

        // Read file size
        let mut size_bytes = [0u8; 8];
        recv.read_exact(&mut size_bytes).await?;
        let file_size = u64::from_le_bytes(size_bytes);

        // Receive and write file content
        let mut file = File::create(&output_path).await?;
        let mut remaining = file_size;
        while remaining > 0 {
            let to_read = std::cmp::min(remaining, CHUNK_SIZE as u64) as usize;
            let mut buffer = vec![0u8; to_read];
            let bytes_read = recv.read(&mut buffer).await?;
            if bytes_read == None {
                return Err(anyhow::anyhow!("Unexpected end of stream"));
            }
            if let Some(bytes_read) = bytes_read {
                file.write_all(&buffer[..bytes_read]).await?;
                remaining -= bytes_read as u64;
                total_bytes += bytes_read;
            }
        }

        println!(
            "Received file: {} ({} bytes)",
            output_path.display(),
            file_size
        );
    }

    println!("All render results received. Total bytes: {}", total_bytes);
    Ok(())
}
