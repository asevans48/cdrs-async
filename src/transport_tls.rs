// use std::net;
use std::io::{IoSlice, IoSliceMut};
use std::marker::Unpin;
use std::task::Poll;

use async_std::io;
use async_std::io::{Read, Write};
use async_std::net;
use async_std::pin::Pin;
use async_std::task::Context;
use async_tls::{client::TlsStream, TlsConnector};
use async_trait::async_trait;

use super::transport::CDRSTransport;

pub type Stream = TlsStream<net::TcpStream>;

/// Default Tls transport.
pub struct TransportTls {
  stream: Stream,
  _addr: String,
}

impl TransportTls {
  /// Constructs a new `TransportTcp`.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use cdrs::transport::TransportTls;
  /// let addr = "127.0.0.1:9042";
  /// let tcp_transport = TransportTls::new(addr).unwrap();
  /// ```
  pub async fn new(addr: &str, connector: TlsConnector) -> io::Result<TransportTls> {
    let tcp_stream = net::TcpStream::connect(addr).await?;
    let stream = connector.connect(addr, tcp_stream)?.await?;
    Ok(TransportTls {
      stream,
      _addr: addr.to_string(),
    })
  }
}

impl Unpin for TransportTls {}

impl Read for TransportTls {
  fn poll_read(
    mut self: Pin<&mut Self>,
    cx: &mut Context,
    buf: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.stream).poll_read(cx, buf)
  }

  fn poll_read_vectored(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    bufs: &mut [IoSliceMut<'_>],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.stream).poll_read_vectored(cx, bufs)
  }
}

impl Write for TransportTls {
  fn poll_write(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.stream).poll_write(cx, buf)
  }

  fn poll_write_vectored(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    bufs: &[IoSlice<'_>],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.stream).poll_write_vectored(cx, bufs)
  }

  fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.stream).poll_flush(cx)
  }

  fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.stream).poll_close(cx)
  }
}

#[async_trait]
impl CDRSTransport for TransportTls {
  // FIXME:
  // async fn try_clone(&self) -> io::Result<TransportTls> {
  //   // TODO:
  //   todo!()
  // }

  fn close(&mut self, close: net::Shutdown) -> io::Result<()> {
    self.stream.get_mut().shutdown(close)
  }

  fn is_alive(&self) -> bool {
    self.stream.get_ref().peer_addr().is_ok()
  }
}