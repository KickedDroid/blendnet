mod lib;

use std::{error::Error, path::PathBuf, process::Command, str::FromStr};

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
use iroh_net::{key::SecretKey, relay::RelayMode, Endpoint};
use tracing::{debug, info};

// An example ALPN that we are using to communicate over the `Endpoint`
const EXAMPLE_ALPN: &[u8] = b"n0/iroh/examples/magic/0";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //tracing_subscriber::fmt::init();
    println!("\nlisten example!\n");
    let secret_key = SecretKey::generate();
    println!("secret key: {secret_key}");

    // Build a `Endpoint`, which uses PublicKeys as node identifiers, uses QUIC for directly connecting to other nodes, and uses the relay protocol and relay servers to holepunch direct connections between nodes when there are NATs or firewalls preventing direct connections. If no direct connection can be made, packets are relayed over the relay servers.
    let endpoint = Endpoint::builder()
        // The secret key is used to authenticate with other nodes. The PublicKey portion of this secret key is how we identify nodes, often referred to as the `node_id` in our codebase.
        .secret_key(secret_key)
        // set the ALPN protocols this endpoint will accept on incoming connections
        .alpns(vec![EXAMPLE_ALPN.to_vec()])
        // `RelayMode::Default` means that we will use the default relay servers to holepunch and relay.
        // Use `RelayMode::Custom` to pass in a `RelayMap` with custom relay urls.
        // Use `RelayMode::Disable` to disable holepunching and relaying over HTTPS
        // If you want to experiment with relaying using your own relay server, you must pass in the same custom relay url to both the `listen` code AND the `connect` code
        .relay_mode(RelayMode::Default)
        // you can choose a port to bind to, but passing in `0` will bind the socket to a random available port
        .bind(0)
        .await?;

    let me = endpoint.node_id();
    println!("node id: {me}");
    println!("node listening addresses:");

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
        .expect("should be connected to a relay server, try calling `endpoint.local_endpoints()` or `endpoint.connect()` first, to ensure the endpoint has actually attempted a connection before checking for the connected relay server");
    println!("node relay server url: {relay_url}");
    println!("\nin a separate terminal run:");

    println!(
        "\tcargo run --example connect -- --node-id {me} --addrs \"{local_addrs}\" --relay-url {relay_url}\n"
    );
    // accept incoming connections, returns a normal QUIC connection
    while let Some(mut conn) = endpoint.accept().await {
        let alpn = conn.alpn().await?;
        let conn = conn.await?;
        let node_id = iroh_net::endpoint::get_remote_node_id(&conn)?;
        info!(
            "new connection from {node_id} with ALPN {} (coming from {})",
            String::from_utf8_lossy(&alpn),
            conn.remote_address()
        );

        // spawn a task to handle reading and writing off of the connection
        tokio::spawn(async move {
            // accept a bi-directional QUIC connection
            // use the `quinn` APIs to send and recv content
            let (mut send, mut recv) = conn.accept_bi().await?;
            debug!("accepted bi stream, waiting for data...");
            let message = recv.read_to_end(100).await?;
            let message = String::from_utf8(message)?;
            println!("received: {message}");

            let message = format!("hi! you connected to {me}. bye bye");
            send.write_all(message.as_bytes()).await?;
            // call `finish` to close the connection gracefully
            send.finish().await?;

            Ok::<_, anyhow::Error>(())
        });
    }
    // stop with SIGINT (ctrl-c)

    Ok(())
}