use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use tokio::{
    net::UnixListener,
    sync::{
        broadcast,
        mpsc::{self, Receiver, Sender},
        oneshot,
    },
    time::sleep,
};

use crate::messages::MainMessage;
use crate::soulseek::client::{peer_connections, server_connection};
use crate::soulseek::protocol::*;
mod messages;
mod soulseek;

const LOCALHOST_ADDRESS: &'static str = "127.0.0.1";
//const SOULFIND_ADDRESS: &'static str = "127.0.0.1:2242";

// Main thread:
// - creates and holds handles to server and peer threads,
// - manages channels of these threads and works with their info,
// - handles Unix socket to communicate with klymene_tui,
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (main_sender, mut main_receiver) = mpsc::channel(512);
    let (server_sender, server_receiver) = mpsc::channel(512);
    let (peer_sender, peer_receiver) = mpsc::channel(512);
    // let socket = UnixListener::bind("/run/klymene/control.sock")?;

    // Handles communication with Soulseek authority.
    let server_connection_thread =
        tokio::spawn(server_connection(server_receiver, main_sender.clone()));

    // Handles peer connections.
    let peer_connections_thread = tokio::spawn(peer_connections(
        (peer_sender.clone(), peer_receiver),
        main_sender.clone(),
    ));
    let socket = Arc::new(SocketAddr::new(LOCALHOST_ADDRESS.parse().unwrap(), 2242));
    let request = Arc::new(LoginRequest {
        username: "username".into(),
        password: "password".into(),
        version_number: 177,
        hash: "d51c9a7e9353746a6020f9602d452929".into(),
        minor_version: 1,
    });

    loop {
        tokio::select! {
            Some(msg) = main_receiver.recv() => {
                println!("{:?}", msg);
                match msg {
                    MainMessage::ServerReady => {
                        println!("Requesting initial connection");
                        server_sender.send(messages::ServerMessage::Connect(socket.clone())).await.unwrap();
                    },
                    MainMessage::ServerConnected(addr) => {
                        println!("Successfully connected to Soulseek server: {}", addr);
                        server_sender.send(messages::ServerMessage::Login(request.clone())).await.unwrap();
                    }
                    MainMessage::ServerConnectionFailed => {
                        panic!("Soulseek connection failed!");
                    },
                    MainMessage::ServerLoginSuccess(res) => {

                    },
                    MainMessage::ServerLoginFail(res) => {

                    },
                    MainMessage::ServerDisconnected => break,
                    MainMessage::ServerShuttingDown => break,
                }
            }
        }
    }

    // Don't think these should panic, just log.
    server_connection_thread
        .await
        .expect("awaiting the server thread")
        .expect("server thread returned an error");
    peer_connections_thread
        .await
        .expect("awaiting the peers thread")
        .expect("peers thread returned an error");
    Ok(())
}
