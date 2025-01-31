use ractor::Actor;
use ractor_cluster::NodeServer;
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


#[tokio::main]
async fn main() {
    let cookie = "cookie".to_string(); // Shared cookie for authentication
    let server_port = 4000; // Port for the server
    let hostname = "localhost".to_string();

    // Create and start the NodeServer actor
    let server = NodeServer::new(
        server_port,
        cookie.clone(),
        "server_node".to_string(),
        hostname.clone(),
        None,
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
