use std::time::Duration;

use tokio::{
    sync::{
        broadcast,
        mpsc::{self, Receiver, Sender},
        oneshot,
    },
    time::sleep,
};

use crate::slsk::client::{peer_connections, server_connection};

mod slsk;

// Main thread:
// - creates and holds handles to server and peer threads,
// - manages channels of these threads and works with their info,
// - handles Unix socket to communicate with klymene_tui,
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // As explained above, this isn't the way it should work, we won't oneshot Vecs of bytes back
    // and forth. There will be just one mpsc channel, server thread and peer thread will receive
    // tasks, threads will act on them. There may be times when servers send a task back to main,
    // so main will have to coordinate.
    let (main_sender, mut main_receiver) = mpsc::channel(1024);
    let (server_sender, server_receiver) = mpsc::channel(1024);
    let (peer_sender, peer_receiver) = mpsc::channel(1024);

    let server_connection_thread = tokio::spawn(server_connection(
        (server_sender.clone(), server_receiver),
        main_sender.clone(),
        peer_sender.clone(),
    ));
    let peer_connections_thread = tokio::spawn(peer_connections(
        (peer_sender.clone(), peer_receiver),
        main_sender.clone(),
        server_sender.clone(),
    ));

    sleep(Duration::from_secs(5)).await;
    main_receiver.close();

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
