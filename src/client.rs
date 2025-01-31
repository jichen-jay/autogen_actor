use ractor::Actor;
use ractor_cluster::{NodeServer, client_connect};

#[tokio::main]
async fn main() {
    let cookie = "cookie".to_string(); // Shared cookie for authentication
    let client_port = 4001; // Port for this client node (not strictly necessary)
    let server_host = "localhost"; // Hostname of the server
    let server_port = 4000; // Port of the server

    // Create a NodeServer actor for the client
    let client = NodeServer::new(
        client_port,
        cookie.clone(),
        "client_node".to_string(),
        "localhost".to_string(),
        None,
        None,
    );

    println!("Starting Client NodeServer on port {}", client_port);

    let (client_actor, _) = Actor::spawn(None, client, ())
        .await
        .expect("Failed to start Client NodeServer");

    println!("Attempting to connect to server at {}:{}", server_host, server_port);

    // Connect to the remote server
    if let Err(error) = client_connect(&client_actor, format!("{server_host}:{server_port}")).await {
        eprintln!("Failed to connect to server with error: {error}");
    } else {
        println!("Successfully connected to the server!");
    }
    
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // #changed: Added delay for session initialization

    // Query active sessions from NodeServer
    if let Ok(sessions) = ractor::call_t!(
        client_actor,
        ractor_cluster::NodeServerMessage::GetSessions,
        200
    ) {
        for session in sessions.into_values() {
            // Check authentication state of each session
            match ractor::call_t!(
                session.actor,
                ractor_cluster::NodeSessionMessage::GetAuthenticationState,
                200
            ) {
                Ok(true) => {
                    println!("Session is authenticated!"); // #changed: Added success message for authenticated state
                }
                Ok(false) => {
                    println!("Session is not authenticated yet."); // #changed: Added message for unauthenticated state
                }
                Err(err) => {
                    println!("Failed to query session authentication state: {err}"); // #changed: Added error handling for failed queries
                }
            }
        }
    } else {
        println!("Failed to retrieve sessions from NodeServer."); // #changed: Added error handling for session retrieval failure
    }
}
