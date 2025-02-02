use autogen_actor::messages::{
    ClusterMessage, CompletionUsage, Content, LlamaResponseMessage, Role, ToolCall,
};
use ractor::Actor;
use ractor_cluster::{client_connect, NodeServer};

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

    println!(
        "Attempting to connect to server at {}:{}",
        server_host, server_port
    );

    // Connect to the remote server
    if let Err(error) = client_connect(&client_actor, format!("{server_host}:{server_port}")).await
    {
        eprintln!("Failed to connect to server with error: {error}");
    } else {
        println!("Successfully connected to the server!");
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // #changed: Added delay for session initialization

    if let Ok(sessions) = ractor::call_t!(
        client_actor,
        ractor_cluster::NodeServerMessage::GetSessions,
        200
    ) {
        for (_, session) in sessions {
            // Create a sample message
            let tool_call = ToolCall {
                name: "example_tool".to_string(),
                arguments: Some(HashMap::from([
                    ("param1".to_string(), "value1".to_string()),
                    ("param2".to_string(), "value2".to_string()),
                ])),
            };

            let message = LlamaResponseMessage {
                content: Content::ToolCall(tool_call),
                role: Role::User,
                usage: CompletionUsage {
                    prompt_tokens: 10,
                    completion_tokens: 20,
                    total_tokens: 30,
                },
            };

            let cluster_message = ClusterMessage::LlamaResponse(message);

            // Send message to server
            if let Err(err) = session
                .actor
                .cast(ractor_cluster::NodeSessionMessage::SendMessage(
                    bincode::serialize(&cluster_message).unwrap(),
                ))
            {
                println!("Failed to send message: {}", err);
            }
        }
    }
}
