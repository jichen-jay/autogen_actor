use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::{client_connect, NodeServerMessage};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Configuration for the client
    let server_port = 8080;
    let server_host = "localhost".to_string();
    let cookie = "cookie".to_string();

    // Create a client actor
    let (client_ref, handle) = Actor::spawn(None, ClientActor, ())
        .await
        .expect("Failed to start client");

    // Connect to the server
    tracing::info!("Connecting to remote NodeServer at {}:{}", server_host, server_port);
    if let Err(error) = client_connect(&client_ref, format!("{}:{}", server_host, server_port)).await {
        tracing::error!("Failed to connect with error {error}");
        return;
    } else {
        tracing::info!("Client connected to NodeServer");
    }

    // Wait for some time to allow for communication
    sleep(Duration::from_secs(5)).await;

    // Stop the client actor
    client_ref.stop(None);
    handle.await.unwrap();
}

struct ClientActor;

#[cfg_attr(feature = "async-trait", ractor::async_trait)]
impl Actor for ClientActor {
    type Msg = NodeServerMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        println!("Client started");
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
