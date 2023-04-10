use std::sync::Arc;

use async_chat::utils::{self, ChatResult};
use async_chat::FromClient;
use async_std::io;
use async_std::net;
use async_std::prelude::*;

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

fn parse_command(line: &str) -> Option<FromClient> {
    Some(FromClient::Post {
        group_name: Arc::new("test".to_string()),
        message: Arc::new("test msg".to_string()),
    })
}
fn main() {
    println!("client running");
}
