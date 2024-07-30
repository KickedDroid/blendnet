use std::{path::PathBuf, str::FromStr};
use anyhow::Context;
use blendnet::{RenderEngine, RenderTask};
use iroh_net::endpoint::RecvStream;
use iroh_net::NodeAddr;
use iroh_net::{key::SecretKey, relay::RelayMode, Endpoint, NodeId};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn spawn() -> anyhow::Result<()> {
    let server_id = NodeId::from_str("server_node_id_here")?;
    let server_addr = NodeAddr::new(server_id);
    let relay_url = "relay_url_here";

    let secret_key = SecretKey::generate();
    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![b"n0/iroh/render/0".to_vec()])
        .relay_mode(RelayMode::Default)
        .bind(0)
        .await?;

    let conn = endpoint.connect(server_addr, b"no/iroh/render/0").await?;
    let (mut send, mut recv) = conn.open_bi().await?;

    // Receive Blend file
    let blend_file_path = PathBuf::from("received_file.blend");
    receive_file(&mut recv, &blend_file_path).await?;

    // Perform rendering
    let render_task = RenderTask {
        id: "1".to_string(),
        blend_file: blend_file_path,
        start_frame: 0,
        end_frame: 1,
        render_engine: RenderEngine::Cycles,
        threads: 1,
    };

    let render_result = execute_render(&render_task)?;

    // Send render completion confirmation
    send.write_all(b"Render completed successfully").await?;
    send.finish().await?;

    println!("Render completed: {:?}", render_result);

    Ok(())
}

async fn receive_file(recv: &mut RecvStream, file_path: &PathBuf) -> anyhow::Result<()> {
    let mut file = File::create(file_path).await?;
    let mut buffer = Vec::new();
    recv.read_to_end(100000).await?;
    file.write_all(&buffer).await?;
    Ok(())
}


fn execute_render(task: &RenderTask) -> anyhow::Result<()> {
    
    println!("Rendering file: {:?}", task.blend_file);
    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}