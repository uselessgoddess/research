use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::Deref;

use bytes::Bytes;

use crate::{FromRequest, RequestContext, rpc};

pub struct Archive<M> {
    bytes: Bytes,
    _marker: PhantomData<M>,
}

impl<M: rpc::Archive> Archive<M> {
    pub fn new(bytes: Bytes) -> rpc::Result<Self> {
        let _ = M::access_bytes(&bytes)?;
        Ok(Self { bytes, _marker: PhantomData })
    }

    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    pub fn deserialize(&self) -> M
    where
        M: rpc::Deserialize<rpc::Error>,
    {
        M::deserialize(self.deref())
            .expect("structural integrity guaranteed by Archive::new; native conversion failed")
    }
}

impl<M: rpc::Archive> Deref for Archive<M> {
    type Target = M::Archived;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Archive::new` checked before `access_unchecked`
        unsafe { M::access_unchecked(&self.bytes) }
    }
}

impl<S: Send + Sync + 'static, M: rpc::Message> FromRequest<S> for Archive<M> {
    type Reject = rpc::Error;

    async fn from_request(ctx: &RequestContext<S>) -> Result<Self, Self::Reject> {
        Archive::new(ctx.frame.payload.clone())
    }
}

impl<M: Debug + rpc::Archive<Archived: Debug>> Debug for Archive<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}
