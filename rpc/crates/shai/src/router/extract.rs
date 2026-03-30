use std::convert::Infallible;
use std::marker::PhantomData;

use bytes::Bytes;

use crate::router::RequestContext;
use crate::rpc;

pub trait FromRequest<S>: Sized + Send + 'static {
    type Reject: Into<rpc::Error>;

    fn from_request(
        ctx: &RequestContext<S>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Reject>> + Send;
}

pub struct State<S>(pub S);

impl<S: Clone + Send + Sync + 'static> FromRequest<S> for State<S> {
    type Reject = Infallible;

    async fn from_request(ctx: &RequestContext<S>) -> Result<Self, Self::Reject> {
        Ok(State(ctx.state.clone()))
    }
}

/// Like [`Archive`], but skips rkyv byte-validation (`CheckBytes`) for performance.
///
/// # Safety
/// Only use this extractor when the sender is fully trusted (e.g. an internal
/// service you control). Receiving malformed bytes from an untrusted source
/// while using this extractor is undefined behaviour.
pub struct Unchecked<M> {
    bytes: Bytes,
    _marker: PhantomData<M>,
}

impl<M: rpc::Archive> Unchecked<M> {
    /// # Safety
    /// Only use this method when the sender is fully trusted (e.g. an internal
    /// service you control). Receiving malformed bytes from an untrusted source
    /// while using this method is undefined behaviour.
    #[inline(always)]
    pub unsafe fn access(&self) -> &M::Archived {
        unsafe { M::access_unchecked(&self.bytes) }
    }
}

impl<S: Send + Sync + 'static, M: rpc::Message> FromRequest<S> for Unchecked<M> {
    type Reject = Infallible;

    async fn from_request(ctx: &RequestContext<S>) -> Result<Self, Self::Reject> {
        Ok(Unchecked { bytes: ctx.frame.payload.clone(), _marker: PhantomData })
    }
}

pub struct Rpc<M>(pub M);

impl<S: Send + Sync + 'static, M> FromRequest<S> for Rpc<M>
where
    M: rpc::Message + rpc::Archive + rpc::Deserialize<rpc::Error>,
{
    type Reject = rpc::Error;

    async fn from_request(ctx: &RequestContext<S>) -> Result<Self, Self::Reject> {
        let archived = M::access_bytes(&ctx.frame.payload)?;
        let message = M::deserialize(archived).map_err(|_| rpc::Error::Decode)?;
        Ok(Rpc(message))
    }
}

pub struct Extension<T>(pub T);

impl<S: Send + Sync + 'static, T: Clone + Send + Sync + 'static> FromRequest<S> for Extension<T> {
    type Reject = rpc::Rejection;

    async fn from_request(ctx: &RequestContext<S>) -> Result<Self, Self::Reject> {
        ctx.peer
            .get_extension::<T>()
            .map(Extension)
            .ok_or_else(|| rpc::Rejection::MissingExtension(std::any::type_name::<T>()))
    }
}
