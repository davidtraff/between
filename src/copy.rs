use anyhow::Result;
use bytes::BytesMut;
use std::time::{SystemTime, UNIX_EPOCH};
use log::info;
use tokio::io::split;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::broadcast::Sender;

const TCP_BUFFER_SIZE: usize = 16 * 1024;

#[derive(Clone, Copy)]
pub enum Source {
    Client,
    Server,
}

#[derive(Clone)]
pub struct Chunk {
    pub source: Source,
    pub data: bytes::Bytes,
    pub at: u64,
}

pub async fn copy_stream(
    server: TcpStream,
    client: TcpStream,
    sender: Sender<Chunk>,
) -> Result<()> {
    let (server_reader, server_writer) = split(server);
    let (client_reader, client_writer) = split(client);

    let mut server_to_client = tokio::spawn(copy_pair(
        server_reader,
        client_writer,
        Source::Server,
        sender.clone(),
    ));
    let mut client_to_server = tokio::spawn(copy_pair(
        client_reader,
        server_writer,
        Source::Client,
        sender,
    ));

    tokio::select! {
        res1 = &mut server_to_client => {
            client_to_server.abort();
            res1??;
        }
        res2 = &mut client_to_server => {
            server_to_client.abort();
            res2??;
        }
    }

    Ok(())
}

async fn copy_pair<R, W>(
    mut reader: ReadHalf<R>,
    mut writer: WriteHalf<W>,
    source: Source,
    sender: Sender<Chunk>,
) -> Result<()>
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
    W: tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    let mut buffer = BytesMut::with_capacity(TCP_BUFFER_SIZE);

    loop {
        buffer.clear();

        let read = reader.read_buf(&mut buffer).await?;
        let now = SystemTime::now();

        if read == 0 {
            break;
        }

        let data = buffer.split().freeze();

        writer.write_all(&data).await?;

        let at = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let _ = sender.send(Chunk { source, data, at });
    }

    info!("Read EOF. Resetting...");

    writer.shutdown().await?;

    info!("Dropped writer");

    Ok(())
}
