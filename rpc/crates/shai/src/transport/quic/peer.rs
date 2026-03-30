use std::io;
use std::net::SocketAddr;

use futures::StreamExt;
use quinn::ConnectionError;
use tokio_util::codec::FramedRead;
use tower::{Service, ServiceExt};
use tracing::{Instrument, debug, error, info_span, warn};

use super::Error;
use crate::IntoTransport;
use crate::router::Router;
use crate::rpc::Frame;
use crate::transport::Extensions;
use crate::transport::codec::{self, FrameCodec};

/// Write a [`Frame`] to a QUIC send stream without an intermediate encoding buffer.
async fn write_frame(send: &mut quinn::SendStream, frame: Frame) -> Result<(), Error> {
    let payload_len = frame.payload.len();
    if payload_len > FrameCodec::DEFAULT_MAX_SIZE {
        return Err(Error::PayloadTooLarge(payload_len));
    }

    let mut header = [0u8; FrameCodec::HEADER_SIZE];
    codec::encode_header(&frame, payload_len, &mut header);
    send.write_all(&header).await?;
    send.write_chunk(frame.payload).await?;
    send.finish()?;
    Ok(())
}

#[tracing::instrument(
    level = "trace",
    skip(router, peer, send, recv),
    fields(
        peer_id = peer.id(),
        stream_id = ?send.id(),
    )
)]
async fn serve_incoming_stream<S>(
    mut router: Router<S>,
    peer: crate::Peer,
    mut send: quinn::SendStream,
    recv: quinn::RecvStream,
) where
    S: Clone + Send + Sync + 'static,
{
    let mut framed_recv = FramedRead::new(recv, FrameCodec::new());

    let frame = match framed_recv.next().await {
        Some(Ok(f)) => f,
        Some(Err(e)) => {
            warn!(error = %e, "failed to decode incoming frame");
            return;
        }
        None => return,
    };

    if let Err(e) = router.ready().await {
        error!(error = %e, "router not ready");
        return;
    }

    tokio::select! {
        biased;
        result = router.call((peer, frame)) => {
            match result {
                Ok(resp_frame) => {
                    if let Err(e) = write_frame(&mut send, resp_frame).await {
                        warn!(error = %e, "failed to write RPC response");
                    }
                }
                Err(e) => error!(error = %e, "router error"),
            }
        }
        stopped = send.stopped() => {
            if let Err(e) = stopped {
                debug!(error = %e, "send stream stopped() finished with error");
            }
            debug!("client stopped receive side; in-flight handler dropped if still running");
        }
    }
}

pub struct Endpoint {
    inner: quinn::Endpoint,
}

impl Endpoint {
    pub fn server(addr: SocketAddr, config: quinn::ServerConfig) -> io::Result<Self> {
        let inner = quinn::Endpoint::server(config, addr)?;
        Ok(Self { inner })
    }

    pub fn client(addr: SocketAddr, config: quinn::ClientConfig) -> io::Result<Self> {
        let mut inner = quinn::Endpoint::client(addr)?;
        inner.set_default_client_config(config);
        Ok(Self { inner })
    }

    pub fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        self.inner.local_addr()
    }

    pub async fn connect(&self, addr: SocketAddr, name: &str) -> Result<Peer, super::Error> {
        let conn = self.inner.connect(addr, name)?.await?;
        Ok(Peer { conn, ext: Extensions::new() })
    }

    /// Accept the next incoming connection. `None` if the endpoint is closed.
    pub async fn accept(&self) -> Option<Result<Peer, super::Error>> {
        let incoming = self.inner.accept().await?;
        Some(incoming.await.map(|conn| Peer { conn, ext: Extensions::new() }).map_err(Into::into))
    }

    /// Accept loop with a per-connection [`Peer::serve_incoming`] task. For lifecycle hooks,
    /// prefer [`Self::accept`] and spawning yourself.
    pub async fn serve<S>(&self, router: Router<S>)
    where
        S: Clone + Send + Sync + 'static,
    {
        while let Some(incoming) = self.inner.accept().await {
            let router = router.clone();
            tokio::spawn(async move {
                match incoming.await {
                    Ok(conn) => {
                        // Errors are logged inside `serve_incoming`; nothing to do here but finish the task.
                        let _ = Peer { conn, ext: Extensions::new() }.serve_incoming(router).await;
                    }
                    Err(e) => tracing::error!("Connection failed: {e}"),
                }
            });
        }
    }
}

#[derive(Clone)]
pub struct Peer {
    pub conn: quinn::Connection,
    pub ext: Extensions,
}

impl Peer {
    pub(crate) async fn exchange_frame(&self, frame: Frame) -> crate::Result<Frame> {
        let (mut send, recv) =
            self.conn.open_bi().await.map_err(Error::Connection).into_transport()?;
        write_frame(&mut send, frame).await.into_transport()?;

        FramedRead::new(recv, FrameCodec::new())
            .next()
            .await
            .unwrap_or_else(|| Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Stream closed")))
            .into_transport()
    }

    /// Runs until the QUIC connection is closed. Does not spawn; use `tokio::spawn` if needed.
    pub async fn serve_incoming<S>(self, router: Router<S>) -> Result<(), Error>
    where
        S: Clone + Send + Sync + 'static,
    {
        let peer = crate::Peer::from(self.clone());
        let span = info_span!("quic_peer", peer_id = self.conn.stable_id() as u64);

        async move {
            loop {
                let (send_stream, recv_stream) = match self.conn.accept_bi().await {
                    Ok(streams) => streams,
                    Err(ConnectionError::ApplicationClosed(_) | ConnectionError::LocallyClosed) => {
                        debug!("QUIC connection closed gracefully");
                        return Ok(());
                    }
                    Err(err) => {
                        warn!(error = %err, "QUIC accept_bi failed");
                        return Err(Error::Connection(err));
                    }
                };

                let (peer, router) = (peer.clone(), router.clone());
                tokio::spawn(
                    serve_incoming_stream(router, peer, send_stream, recv_stream)
                        .instrument(tracing::Span::current()),
                );
            }
        }
        .instrument(span)
        .await
    }
}
