use crate::protocol::{Command, Response};
use anyhow::Result;
use serde_json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub async fn send_command(addr: &str, cmd: Command) -> Result<Response> {
    let mut stream = TcpStream::connect(addr).await?;
    let cmd_json = serde_json::to_string(&cmd)?;
    stream.write_all(cmd_json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;

    let mut buf_reader = BufReader::new(stream);
    let mut response_line = String::new();
    buf_reader.read_line(&mut response_line).await?;

    let response: Response = serde_json::from_str(&response_line)?;
    Ok(response)
}

pub async fn listen_and_serve<F>(addr: &str, handler: F) -> Result<()>
where
    F: Fn(Command) -> Response + Send + Sync + 'static + Clone,
{
    let listener = TcpListener::bind(addr).await?;
    println!("Aletheia Memory Node listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let handler = handler.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, handler).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client<F>(socket: TcpStream, handler: F) -> Result<()>
where
    F: Fn(Command) -> Response,
{
    let (reader, mut writer) = socket.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    while buf_reader.read_line(&mut line).await? > 0 {
        let cmd: Command = serde_json::from_str(&line)?;
        let response = handler(cmd);
        let response_json = serde_json::to_string(&response)?;
        writer.write_all(response_json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        line.clear();
    }

    Ok(())
}
