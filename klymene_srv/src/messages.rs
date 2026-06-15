/// This module contains the messages that are sent between the threads.
use crate::soulseek::protocol::*;
use std::sync::Arc;
#[derive(Debug)]
pub enum MainMessage {
    ServerReady,
    ServerConnected(std::net::SocketAddr),
    ServerDisconnected,
    ServerConnectionFailed, // @Todo: Reason
    ServerLoginSuccess(LoginSuccess),
    ServerLoginFail(LoginFailure),
    ServerShuttingDown,
}

#[derive(Debug)]
pub enum PeerMessage {
    Ready,
    ShuttingDown,
}

#[derive(Debug)]
pub enum ServerMessage {
    Connect(Arc<std::net::SocketAddr>),
    Disconnect,
    Login(Arc<LoginRequest>),
    Shutdown,
}
