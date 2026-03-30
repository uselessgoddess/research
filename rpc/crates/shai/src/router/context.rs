use std::marker::PhantomData;

use bytes::Bytes;

use crate::Message;
use crate::router::FromRequest;
use crate::rpc::{self, Frame};
use crate::util::BoxFuture;

pub struct RequestContext<S> {
    pub peer: crate::Peer,
    pub frame: Frame,
    pub state: S,
}

pub trait ErasedHandler<S>: Send + Sync {
    fn handle(&self, ctx: RequestContext<S>) -> BoxFuture<'static, rpc::Result<Bytes>>;
}

pub trait Handler<M: Message, S, T>: Clone + Send + Sync + 'static {
    type Future: Future<Output = rpc::Result<M::Response>> + Send + 'static;

    fn call(self, ctx: RequestContext<S>) -> Self::Future;
}

pub struct TypedHandler<M, H, T> {
    handler: H,
    _marker: PhantomData<fn() -> (M, T)>,
}

impl<M, H, T> TypedHandler<M, H, T> {
    pub fn new(handler: H) -> Self {
        Self { handler, _marker: PhantomData }
    }
}

impl<M, S, H, T> ErasedHandler<S> for TypedHandler<M, H, T>
where
    M: Message,
    H: Handler<M, S, T>,
    S: Send + Sync + 'static,
{
    fn handle(&self, ctx: RequestContext<S>) -> BoxFuture<'static, rpc::Result<Bytes>> {
        let fut = self.handler.clone().call(ctx);

        Box::pin(async move { crate::rpc::Serialize::serialize_to_bytes(&fut.await?) })
    }
}

macro_rules! impl_handler {
    ( $($ty:ident),* ) => {
        #[allow(non_snake_case, unused_variables)]
        impl<M, S, F, Fut, $($ty,)*> Handler<M, S, ($($ty,)*)> for F
        where
            M: Message,
            S: Send + Sync + 'static,
            F: Fn($($ty,)*) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = rpc::Result<M::Response>> + Send,
            $( $ty: FromRequest<S> + Send, )*
        {
            type Future = BoxFuture<'static, rpc::Result<M::Response>>;

            fn call(self, ctx: RequestContext<S>) -> Self::Future {
                Box::pin(async move {
                    $(
                        let $ty = $ty::from_request(&ctx).await.map_err(Into::into)?;
                    )*
                    (self)($($ty,)*).await
                })
            }
        }
    };
}

impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);
