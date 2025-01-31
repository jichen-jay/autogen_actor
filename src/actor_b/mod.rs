use crate::{ClientMessage, ServerActor, ServerMessage};
use ractor::{Actor, ActorProcessingErr, ActorRef};

impl Actor for ServerActor {
    type Msg = ServerMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        println!("Server started");
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            ServerMessage::RegisterClient(client_id) => {
                let (client_actor, _) =
                    ractor::Actor::spawn(None, ClientActor { id: client_id }, ())
                        .await
                        .expect("Failed to spawn client actor");
                self.clients.lock().await.insert(client_id, client_actor);
                println!("Registered client {}", client_id);
            }
            ServerMessage::SendMessage(client_id, msg) => {
                if let Some(client) = self.clients.lock().await.get(&client_id) {
                    client.cast(ClientMessage::Receive(msg)).unwrap();
                } else {
                    println!("Client {} not found", client_id);
                }
            }
            ServerMessage::DisconnectClient(client_id) => {
                self.clients.lock().await.remove(&client_id);
                println!("Disconnected client {}", client_id);
            }
        }
        Ok(())
    }
}

// Client Actor
struct ClientActor {
    id: u32,
}

impl Actor for ClientActor {
    type Msg = ClientMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        println!("Client {} started", self.id);
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            ClientMessage::Receive(msg) => println!("Client {} received message: {}", self.id, msg),
        }
        Ok(())
    }
}

