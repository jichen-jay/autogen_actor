use ractor::Actor;
use ractor_cluster::NodeServer;
use autogen_actor::messages::{ClusterMessage, Content};

struct SubscriptionLogger;

impl ractor_cluster::NodeEventSubscription for SubscriptionLogger {
    fn node_session_authenicated(&self, ses: ractor_cluster::node::NodeServerSessionInformation) {
        println!("[Logger] Session authenticated: {:?}", ses.peer_addr);
    }
    fn node_session_disconnected(&self, ses: ractor_cluster::node::NodeServerSessionInformation) {
        println!("[Logger] Session disconnected: {:?}", ses.peer_addr);
    }
    fn node_session_opened(&self, ses: ractor_cluster::node::NodeServerSessionInformation) {
        println!("[Logger] Session opened: {:?}", ses.peer_addr);
    }
}

// Create a separate message handler actor
#[derive(Default)]
struct MessageHandler;

#[async_trait::async_trait]
impl ractor::Actor for MessageHandler {
    type Arguments = ();
    type Msg = Vec<u8>;
    type State = ();

    async fn pre_start(&self, _: ActorRef<Self::Msg>, _: Self::Arguments) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(&self, _: ActorRef<Self::Msg>, message: Self::Msg, _: &mut Self::State) -> Result<(), ActorProcessingErr> {
        match bincode::deserialize::<ClusterMessage>(&message) {
            Ok(ClusterMessage::LlamaResponse(response)) => {
                println!("[Handler] Received LlamaResponse:");
                println!("  Role: {:?}", response.role);
                match response.content {
                    Content::Text(text) => println!("  Content: Text({})", text),
                    Content::ToolCall(tool) => println!("  Content: ToolCall({:?})", tool),
                }
                println!("  Usage: {:?}", response.usage);
            }
            Ok(ClusterMessage::Command(cmd)) => {
                println!("[Handler] Received Command: {}", cmd);
            }
            Ok(ClusterMessage::Acknowledgement(ack)) => {
                println!("[Handler] Received Acknowledgement: {}", ack);
            }
            Err(e) => println!("[Handler] Failed to deserialize message: {}", e),
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cookie = "cookie".to_string();
    let server_port = 4000;
    let hostname = "localhost".to_string();

    // Spawn message handler actor
    let (handler_actor, _) = Actor::spawn(None, MessageHandler::default(), ())
        .await
        .expect("Failed to start MessageHandler");

    // Create and start the NodeServer actor with message handler
    let server = NodeServer::new(
        server_port,
        cookie.clone(),
        "server_node".to_string(),
        hostname.clone(),
        Some(handler_actor), // Pass the message handler actor
        None,
    );

    println!("Starting NodeServer on port {}", server_port);

    let (server_actor, handle) = Actor::spawn(None, server, ())
        .await
        .expect("Failed to start NodeServer");

    println!("NodeServer is running. Waiting for client connections...");
    let log_subscriber = Box::new(SubscriptionLogger);
    server_actor
        .cast(ractor_cluster::NodeServerMessage::SubscribeToEvents {
            id: "logger".to_string(),
            subscription: log_subscriber,
        })
        .expect("Failed to subscribe to events");

    // Wait indefinitely (or until interrupted)
    handle.await.unwrap();
}
