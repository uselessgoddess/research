mod context;
pub mod extract;

use std::sync::Arc;
use std::task::{Context, Poll};

use bytes::Bytes;
use context::ErasedHandler;
pub use context::{Handler, RequestContext, TypedHandler};
pub use extract::FromRequest;

use crate::rpc::{self, Flags, MessageId, Status};
use crate::util::BoxFuture;

type Routes<S> = rustc_hash::FxHashMap<MessageId, Arc<dyn ErasedHandler<S>>>;

#[derive(Clone)]
pub struct Router<S> {
    routes: Arc<Routes<S>>,
    state: S,
}

impl<S: Send + Sync + 'static> Router<S> {
    pub fn new(state: S) -> Self {
        Self { routes: Arc::new(Routes::default()), state }
    }

    pub fn route<M, T, H>(mut self, handler: H) -> Self
    where
        M: rpc::Message,
        H: Handler<M, S, T>,
        T: 'static,
    {
        let erased = Arc::new(TypedHandler::<M, H, T>::new(handler));

        Arc::get_mut(&mut self.routes)
            .expect("Cannot add routes after router is shared")
            .insert(M::ID, erased);

        self
    }
}

// TODO: use alternative of `axum::FromRef` to avoid explicit `Clone`
impl<S: Clone + Send + Sync + 'static> tower::Service<(crate::Peer, rpc::Frame)> for Router<S> {
    type Response = rpc::Frame;
    type Error = rpc::Error;
    type Future = BoxFuture<'static, rpc::Result<rpc::Frame>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, (peer, req): (crate::Peer, rpc::Frame)) -> Self::Future {
        let mut frame = rpc::Frame {
            id: req.id.into_response(),
            flags: Flags::EMPTY,
            status: Status::Ok,
            trace_id: req.trace_id,
            payload: Bytes::new(),
        };

        if let Some(handler) = self.routes.get(&req.id) {
            let handler = Arc::clone(handler);
            let ctx = RequestContext { peer, frame: req, state: self.state.clone() };

            Box::pin(async move {
                match handler.handle(ctx).await {
                    Ok(payload) => frame.payload = payload,
                    Err(err) => {
                        frame.status = Status::from(&err);
                        if let rpc::Error::Internal(msg) = err {
                            frame.payload = Bytes::from(msg);
                        }
                    }
                }
                Ok(frame)
            })
        } else {
            frame.status = rpc::Status::NotFound;
            Box::pin(std::future::ready(Ok(frame)))
        }
    }
}
