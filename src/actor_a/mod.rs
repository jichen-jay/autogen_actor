use ractor::{Actor, ActorProcessingErr, ActorRef};
use crate::ServerMessage;



// Client Acto
pub struct ClientActor {
 pub   id: u32,
 pub   server_ref: ActorRef<ServerMessage>,
}

impl Actor for ClientActor {
    type Msg = ();
    type State = ();
    type Arguments = (u32, ActorRef<ServerMessage>);

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (client_id, server_ref) = args;
        println!("Client {} started", client_id);

        // Register with the server
        server_ref.cast(ServerMessage::RegisterClient(client_id)).unwrap();

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        _message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Handle any internal messages if needed
        Ok(())
    }
}

