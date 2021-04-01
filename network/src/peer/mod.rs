use futures::io::{AsyncRead, AsyncWrite};
use futures::stream::{Fuse, FuturesUnordered};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::net::Ipv4Addr;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct NetworkAddress(Vec<Protocol>);

/// A single protocol in the [`NetworkAddress`] protocol stack.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Protocol {
  Ip4(Ipv4Addr),
}

pub struct PeerManager {
  listen_addr: NetworkAddress,
}

pub struct Peer<TSocket> {
  connection: Option<TSocket>,
}

impl<TSocket> Peer<TSocket>
where
  TSocket: AsyncRead + AsyncWrite + Send + 'static,
{
  pub fn new() -> Self {
    unimplemented!();
  }

  pub async fn start(mut self) {
    unimplemented!();
  }
}
