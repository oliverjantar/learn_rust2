use std::sync::Arc;

use async_chat::utils::{self, ChatResult};
use async_chat::FromClient;
use async_chat::FromServer;
use async_std::io;
use async_std::net;
use async_std::prelude::*;
use async_std::task;
use futures_lite::FutureExt;

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\n\
              join GROUP\n\
              post GROUP MESSAGE...\n\
              Type Control-D (Unix) Control-Z (Windows) \
              to close the connection"
    );

    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }
    Ok(())
}

//todo: this is not ready
fn parse_command(line: &str) -> Option<FromClient> {
    Some(FromClient::Post {
        group_name: Arc::new("test".to_string()),
        message: Arc::new("test msg".to_string()),
    })
}

async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            FromServer::Error(message) => {
                print!("error from server: {}", message);
            }
        }
    }
    Ok(())
}

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;

        Ok(())
    })
}
