use std::time::Duration;

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
use crate::slsk::client::{peer_connections, server_connection};
mod messages;
mod slsk;

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
    let server_connection_thread = tokio::spawn(server_connection(
        server_receiver,
        main_sender.clone(),
        peer_sender.clone(),
    ));

    // Handles peer connections.
    let peer_connections_thread = tokio::spawn(peer_connections(
        (peer_sender.clone(), peer_receiver),
        main_sender.clone(),
        server_sender.clone(),
    ));

    loop {
        tokio::select! {
            Some(msg) = main_receiver.recv() => {
                println!("{:?}", msg);
                match msg {
                    MainMessage::ServerConnected => {
                        println!("Successfully connected to Soulseek server.");
                    }
                    MainMessage::ServerConnectionFailed => {
                        panic!("Soulseek connection failed!");
                    }
                    MainMessage::ServerShuttingDown => break,
                    _ => (),
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
