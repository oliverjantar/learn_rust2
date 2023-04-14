use crate::connection::Outbound;
use async_chat::FromServer;
use async_std::task;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{self, Receiver};

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Self {
        let (sender, _receiver) = broadcast::channel(1000);
        Group { name, sender }
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();

        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>) {
        let _ignored = self.sender.send(message);
    }
}

async fn handle_subscriber(
    name: Arc<String>,
    mut receiver: Receiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                FromServer::Error(format!("Dropped {} messages from {}.", n, name))
            }
            Err(RecvError::Closed) => break,
        };

        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}
