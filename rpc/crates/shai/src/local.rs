use std::io;
use std::net::SocketAddr;

use tokio::sync::{mpsc, oneshot};
use tower::{Service, ServiceExt};

use crate::rpc::Frame;
use crate::{Error, Extensions, Router};

type FrameChannel = mpsc::Sender<(Frame, oneshot::Sender<Frame>)>;

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: u64,
    pub addr: SocketAddr,
    pub ext: Extensions,
    tx: Option<FrameChannel>,
}

impl Default for Peer {
    fn default() -> Self {
        Self::new()
    }
}

impl Peer {
    pub fn new() -> Self {
        Self {
            id: 0,
            addr: SocketAddr::from(([127, 0, 0, 1], 0)),
            ext: Extensions::new(),
            tx: None,
        }
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn with_addr(mut self, addr: SocketAddr) -> Self {
        self.addr = addr;
        self
    }

    pub fn connect<S>(mut self, router: Router<S>) -> Self
    where
        S: Clone + Send + Sync + 'static,
    {
        let (tx, mut rx) = mpsc::channel(1024);
        self.tx = Some(tx);

        let peer = crate::Peer::from(self.clone());

        tokio::spawn(async move {
            while let Some((frame, res_tx)) = rx.recv().await {
                let peer = peer.clone();
                let mut svc = router.clone();
                tokio::spawn(async move {
                    if let Ok(svc) = svc.ready().await
                        && let Ok(frame) = svc.call((peer, frame)).await
                    {
                        let _ = res_tx.send(frame);
                    }
                });
            }
        });

        self
    }

    pub(crate) async fn exchange_frame(&self, frame: Frame) -> crate::Result<Frame> {
        let tx = self.tx.as_ref().expect("Fake peer not connected! Use .connect(router)");
        let (res_tx, res_rx) = oneshot::channel();

        tx.send((frame, res_tx)).await.map_err(|_| {
            Error::transport(io::Error::new(io::ErrorKind::BrokenPipe, "Router dropped"))
        })?;

        res_rx.await.map_err(|_| {
            Error::transport(io::Error::new(io::ErrorKind::ConnectionAborted, "Response dropped"))
        })
    }
}
