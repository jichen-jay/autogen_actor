pub mod actor_a;
pub mod actor_b;
pub mod message;

use ractor::{ActorRef};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    RegisterClient(u32),
    SendMessage(u32, String),
    DisconnectClient(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Receive(String),
}
 
pub struct ServerActor {
  pub  clients: Arc<Mutex<HashMap<u32, ActorRef<ClientMessage>>>>,
}