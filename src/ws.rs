use anyhow::Result;
use tokio::sync::broadcast::{Sender, Receiver};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::SinkExt;
use crate::copy::{Chunk, Source};
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

#[derive(Serialize)]
struct WsMessage<'a> {
    source: &'a str,
    data: &'a str,
    at: u64,
}

pub async fn run_ws(listener: TcpListener, tx: Sender<Chunk>) -> Result<()> {
    while let Ok(client) = listener.accept().await {
        let (client, _) = client;
        let rx = tx.subscribe();

        tokio::spawn(run_ws_client(client, rx));
    }
    Ok(())
}

async fn run_ws_client(client: TcpStream, mut rx: Receiver<Chunk>) -> Result<()> {
    let mut ws = accept_async(client).await?;

    loop {
        match rx.recv().await {
            Ok(chunk) => {
                let encoded_data = general_purpose::STANDARD.encode(&chunk.data);
                let msg = WsMessage {
                    source: match chunk.source {
                        Source::Client => "client",
                        Source::Server => "server",
                    },
                    data: &encoded_data,
                    at: chunk.at
                };
                let json = serde_json::to_string(&msg)?;
                ws.send(Message::Text(json.into())).await?;
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                continue;
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                break;
            }
        }
    }

    Ok(())
}

