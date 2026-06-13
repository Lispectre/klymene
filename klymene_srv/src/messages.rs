/// This module contains the messages that are sent between the threads.

#[derive(Debug)]
pub enum MainMessage {
    ServerConnected,
    ServerConnectionFailed, // @Todo: Reason
    ServerShuttingDown,
}

#[derive(Debug)]
pub enum PeerMessage {
    Ready,
    ShuttingDown,
}

#[derive(Debug)]
pub enum ServerMessage {
    Ready,
    ShuttingDown,
}
