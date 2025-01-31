// Copyright (c) Sean Lawlor
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree.

//! Test the authentication handshake, making sure 2 nodes can interconnect together and authenticate
//! with the secret cookie challenge scheme

use clap::Args;

use ractor::concurrency::{sleep, Duration, Instant};
use ractor::Actor;

const AUTH_TIME_ALLOWANCE_MS: u128 = 1500;

/// Configuration
#[derive(Args, Debug, Clone)]
pub struct AuthHandshakeConfig {
    /// Server port
    server_port: u16,
    /// If specified, represents the client to connect to
    client_port: Option<u16>,
    /// If specified, represents the client to connect to
    client_host: Option<String>,
}

struct SubscriptionEventLogger;

impl ractor_cluster::NodeEventSubscription for SubscriptionEventLogger {
    fn node_session_authenicated(&self, ses: ractor_cluster::node::NodeServerSessionInformation) {
        println!(
            "[SubscriptionEventLogger] Node {} ({}) authenticated",
            ses.node_id,
            ses.peer_addr
        );
    }
    fn node_session_disconnected(&self, ses: ractor_cluster::node::NodeServerSessionInformation) {
        println!(
            "[SubscriptionEventLogger] Node {} ({}) disconnected",
            ses.node_id,
            ses.peer_addr
        );
    }
    fn node_session_opened(&self, ses: ractor_cluster::node::NodeServerSessionInformation) {
        println!(
            "[SubscriptionEventLogger] Node {} ({}) opened",
            ses.node_id,
            ses.peer_addr
        );
    }
}

#[tokio::main]
async fn main() {
    let config = AuthHandshakeConfig {
        server_port: 8080,
        client_port: Some(8080),
        client_host: Some("localhost".to_string()),
    };
    let cookie = "cookie".to_string();
    let hostname = "localhost".to_string();

    let server = ractor_cluster::NodeServer::new(
        config.server_port,
        cookie,
        "jaykchen".to_string(),
        hostname.clone(),
        None,
        None,
    );

    println!("Starting NodeServer on port {}", config.server_port);

    let (actor, handle) = Actor::spawn(None, server, ())
        .await
        .expect("Failed to start NodeServer");

    let log_sub = Box::new(SubscriptionEventLogger);
    actor
        .cast(ractor_cluster::NodeServerMessage::SubscribeToEvents {
            id: "logger".to_string(),
            subscription: log_sub,
        })
        .expect("Failed to send log subscription");

    if let (Some(client_host), Some(client_port)) = (config.client_host, config.client_port) {
        println!("Connecting to remote NodeServer at {client_host}:{client_port}");
        if let Err(error) =
            ractor_cluster::client_connect(&actor, format!("{client_host}:{client_port}")).await
        {
            eprintln!("Failed to connect with error {error}");
            print!("-3");
        } else {
            println!("Client connected NodeServer b to NodeServer a");
        }
    }

    let mut err_code = -1;
    println!("Waiting for NodeSession status updates");

    let mut rpc_reply = ractor::call_t!(actor, ractor_cluster::NodeServerMessage::GetSessions, 200);
    let mut tic = None;

    while rpc_reply.is_ok() {
        if let Some(timestamp) = tic {
            let time: Duration = Instant::now() - timestamp;
            if time.as_millis() > AUTH_TIME_ALLOWANCE_MS {
                err_code = -2;
                eprintln!(
                    "The authentcation time has been going on for over > {}ms. Failing the test",
                    time.as_millis()
                );
                break;
            }
        }

        if let Some(item) = rpc_reply
            .unwrap()
            .into_values()
            .collect::<Vec<_>>()
            .first()
            .cloned()
        {
            // we got an actor, track how long it took to auth, maxing out at 500ms
            if tic.is_none() {
                tic = Some(Instant::now());
            }

            let is_authenticated = ractor::call_t!(
                item.actor,
                ractor_cluster::NodeSessionMessage::GetAuthenticationState,
                200
            );
            match is_authenticated {
                Err(err) => {
                    println!("NodeSession returned error on rpc query {err}");
                    break;
                }
                Ok(false) => {
                    // Still waiting
                }
                Ok(true) => {
                    err_code = 0;
                    println!("Authentication succeeded. Exiting test");
                    break;
                }
            }
        }
        // try again
        rpc_reply = ractor::call_t!(actor, ractor_cluster::NodeServerMessage::GetSessions, 200);
    }

    println!("Terminating test - code {err_code}");

    sleep(Duration::from_millis(250)).await;

    // cleanup
    actor.stop(None);
    handle.await.unwrap();

    println!("error: {err_code}");
    
    std::process::exit(err_code)
}
