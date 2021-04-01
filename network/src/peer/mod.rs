use futures::io::{AsyncRead, AsyncWrite};

pub struct Peer<TSocket> {
  connection: Option<TSocket>,
}

impl<TSocket> Peer<TSocket>
where
  TSocket: AsyncRead + AsyncWrite + Send + 'static,
{
  pub fn new(connection: Option<TSocket>) -> Self {
    Peer { connection: None }
  }

  pub async fn start(mut self) {
    unimplemented!();
  }
}
