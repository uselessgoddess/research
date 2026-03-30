use std::convert::Infallible;
use std::net::SocketAddr;

use crate::router::Router;
use crate::rpc::Frame;
use crate::{Archive, Error, Extensions, FromRequest, RequestContext, local, rpc};

#[derive(Clone)]
enum PeerInner {
    Quic(super::quic::Peer),
    Local(local::Peer),
}

#[derive(Clone)]
pub struct Peer {
    inner: PeerInner,
}

impl Peer {
    pub fn id(&self) -> u64 {
        match &self.inner {
            PeerInner::Quic(p) => p.conn.stable_id() as u64,
            PeerInner::Local(p) => p.id,
        }
    }

    pub fn remote_addr(&self) -> SocketAddr {
        match &self.inner {
            PeerInner::Quic(p) => p.conn.remote_address(),
            PeerInner::Local(p) => p.addr,
        }
    }

    pub async fn call<M: rpc::Message>(&self, req: &M) -> crate::Result<Archive<M::Response>> {
        self.request_raw(req, [0; 16]).await
    }

    // TODO: make request builder to avooid this explicit trace id
    pub async fn request_raw<M: rpc::Message>(
        &self,
        req: &M,
        trace_id: [u8; 16],
    ) -> crate::Result<Archive<M::Response>> {
        let payload = req.serialize_to_bytes().map_err(|_| rpc::Error::Encode)?;

        let frame =
            Frame::new(M::ID, rpc::Flags::EMPTY, rpc::Status::Ok, payload).with_trace(trace_id);

        let frame = match &self.inner {
            PeerInner::Quic(p) => p.exchange_frame(frame).await?,
            PeerInner::Local(p) => p.exchange_frame(frame).await?,
        };

        if frame.status != rpc::Status::Ok {
            return Err(Error::Status { status: frame.status, payload: frame.payload });
        }

        Archive::new(frame.payload).map_err(Into::into)
    }

    fn ext(&self) -> &Extensions {
        match &self.inner {
            PeerInner::Quic(p) => &p.ext,
            PeerInner::Local(p) => &p.ext,
        }
    }

    pub fn get_extension<T: Clone + Sync + Send + 'static>(&self) -> Option<T> {
        self.ext().read().get::<T>().cloned()
    }

    pub fn insert_extension<T: Clone + Send + Sync + 'static>(&self, val: T) {
        let _ = self.ext().write().insert(val);
    }
    pub async fn serve<S>(&self, router: Router<S>) -> crate::Result<()>
    where
        S: Clone + Send + Sync + 'static,
    {
        match &self.inner {
            PeerInner::Quic(p) => p
                .clone()
                .serve_incoming(router)
                .await
                .map_err(crate::Error::transport),
            PeerInner::Local(_) => Ok(()),
        }
    }
}

impl From<super::quic::Peer> for Peer {
    fn from(peer: super::quic::Peer) -> Self {
        Self { inner: PeerInner::Quic(peer) }
    }
}

impl From<local::Peer> for Peer {
    fn from(sess: local::Peer) -> Self {
        Self { inner: PeerInner::Local(sess) }
    }
}

impl<S: Send + Sync + 'static> FromRequest<S> for Peer {
    type Reject = Infallible;

    async fn from_request(ctx: &RequestContext<S>) -> Result<Self, Self::Reject> {
        Ok(ctx.peer.clone())
    }
}
