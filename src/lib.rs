#![recursion_limit = "1024"]
#[macro_use]
extern crate enum_primitive_derive;
use num_traits::ToPrimitive;

use async_trait::async_trait;
use futures::channel::{mpsc, oneshot};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};

mod codec;
mod dealer_router;
mod error;
mod message;
mod pub_sub;
mod req_rep;
pub mod util;

#[cfg(test)]
mod tests;

use crate::codec::*;
pub use crate::dealer_router::*;
pub use crate::error::ZmqError;
pub use crate::pub_sub::*;
pub use crate::req_rep::*;
use crate::util::*;
pub use message::*;

pub type ZmqResult<T> = Result<T, ZmqError>;

#[derive(Clone, Copy, Debug, PartialEq, Primitive)]
pub enum SocketType {
    PAIR = 0,
    PUB = 1,
    SUB = 2,
    REQ = 3,
    REP = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL = 7,
    PUSH = 8,
    XPUB = 9,
    XSUB = 10,
    STREAM = 11,
}

impl TryFrom<&str> for SocketType {
    type Error = ZmqError;

    fn try_from(s: &str) -> Result<Self, ZmqError> {
        Ok(match s {
            "PAIR" => SocketType::PAIR,
            "PUB" => SocketType::PUB,
            "SUB" => SocketType::SUB,
            "REQ" => SocketType::REQ,
            "REP" => SocketType::REP,
            "DEALER" => SocketType::DEALER,
            "ROUTER" => SocketType::ROUTER,
            "PULL" => SocketType::PULL,
            "PUSH" => SocketType::PUSH,
            "XPUB" => SocketType::XPUB,
            "XSUB" => SocketType::XSUB,
            "STREAM" => SocketType::STREAM,
            _ => return Err(ZmqError::Codec("Unknown socket type")),
        })
    }
}

impl Display for SocketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketType::PAIR => write!(f, "PAIR"),
            SocketType::PUB => write!(f, "PUB"),
            SocketType::SUB => write!(f, "SUB"),
            SocketType::REQ => write!(f, "REQ"),
            SocketType::REP => write!(f, "REP"),
            SocketType::DEALER => write!(f, "DEALER"),
            SocketType::ROUTER => write!(f, "ROUTER"),
            SocketType::PULL => write!(f, "PULL"),
            SocketType::PUSH => write!(f, "PUSH"),
            SocketType::XPUB => write!(f, "XPUB"),
            SocketType::XSUB => write!(f, "XSUB"),
            SocketType::STREAM => write!(f, "STREAM"),
        }
    }
}

trait MultiPeer: SocketBackend {
    fn peer_connected(
        &self,
        peer_id: &PeerIdentity,
    ) -> (mpsc::Receiver<Message>, oneshot::Receiver<bool>);
    fn peer_disconnected(&self, peer_id: &PeerIdentity);
}

#[async_trait]
trait SocketBackend: Send + Sync {
    async fn message_received(&self, peer_id: &PeerIdentity, message: Message);

    fn socket_type(&self) -> SocketType;
    fn shutdown(&self);
}

#[async_trait]
pub trait Socket: Send {
    async fn send(&mut self, message: ZmqMessage) -> ZmqResult<()>;
    async fn recv(&mut self) -> ZmqResult<ZmqMessage>;
}

#[async_trait]
pub trait SocketFrontend {
    fn new() -> Self;

    /// Opens port described by endpoint and starts a coroutine to accept new connections on it
    async fn bind(&mut self, endpoint: &str) -> ZmqResult<()>;
    async fn connect(&mut self, endpoint: &str) -> ZmqResult<()>;
}

#[async_trait]
pub trait SocketServer {
    async fn accept(&mut self) -> ZmqResult<Box<dyn Socket>>;
}

pub async fn bind(socket_type: SocketType, endpoint: &str) -> ZmqResult<Box<dyn SocketServer>> {
    let listener = TcpListener::bind(endpoint).await?;
    match socket_type {
        SocketType::REP => Ok(Box::new(RepSocketServer { _inner: listener })),
        _ => todo!(),
    }
}

pub async fn proxy(_s1: Box<dyn Socket>, _s2: Box<dyn Socket>) -> ZmqResult<()> {
    todo!()
}
