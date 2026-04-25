use anyhow::Result;
use config::NetworkConfig;
use copy::{Chunk, copy_stream};
use log::{error, info};
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio::time::sleep;

mod config;
mod copy;
mod logging;
mod ws;

async fn run_proxy(tx: Sender<Chunk>, cfg: &NetworkConfig) -> Result<()> {
    let client_listener = TcpListener::bind(&cfg.proxy_listen_address).await?;

    loop {
        info!("Listening for connection on {}", cfg.proxy_listen_address);

        let Ok((client, remote)) = client_listener.accept().await else {
            break;
        };

        info!("Client connected from {}", remote);

        let server = get_server(cfg).await;

        info!("Connected to server at {}", cfg.proxy_connect_address);

        if let Err(e) = copy_stream(server, client, tx.clone()).await {
            error!("Client/server error. Resetting. Reason {}", e);
        }

        info!("Dropping client");
    }

    Ok(())
}

async fn get_server(cfg: &NetworkConfig) -> TcpStream {
    loop {
        match TcpStream::connect(&cfg.proxy_connect_address).await {
            Ok(client) => return client,
            Err(e) => {
                error!(
                    "Could not connect to {}. Retrying in 1s. Reason {}",
                    cfg.proxy_connect_address, e
                );
            }
        }

        sleep(Duration::from_millis(1000)).await;
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging();

    let cfg = config::AppConfig::load()?;
    let (tx, _) = channel(1024);

    // WebSocket listener
    let ws_listener = TcpListener::bind(&cfg.network.ws_address).await?;

    let ws_task = ws::run_ws(ws_listener, tx.clone());
    let proxy_task = run_proxy(tx, &cfg.network);

    tokio::try_join!(ws_task, proxy_task)?;

    Ok(())
}
