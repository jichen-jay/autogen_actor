use ractor::{Actor, ActorProcessingErr, ActorRef};
use ractor_cluster::{NodeServer, NodeServerMessage};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Configuration for the server
    let server_port = 8080;
    let cookie = "cookie".to_string();
    let hostname = "localhost".to_string();

    // Create the server actor
    let server = NodeServer::new(
        server_port,
        cookie,
        "server".to_string(),
        hostname,
        None,
        None,
    );

    tracing::info!("Starting NodeServer on port {}", server_port);
    let (server_ref, handle) = Actor::spawn(None, server, ())
        .await
        .expect("Failed to start NodeServer");

    // Let the server start up
    sleep(Duration::from_millis(100)).await;

    // Wait for some time to allow for client connections
    sleep(Duration::from_secs(60)).await;

    // Stop the server actor
    // server_ref.stop(None);
    handle.await.unwrap();
}
