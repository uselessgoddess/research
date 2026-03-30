pub mod codec;
mod peer;
pub mod quic;

use std::ops::Deref;
use std::sync::Arc;

use parking_lot::RwLock;
pub use peer::Peer;

use crate::util;

// TODO: put mutex into `uitl::Extensions` to avoid 24 byte size
#[derive(Debug, Default, Clone)]
pub struct Extensions(Arc<RwLock<util::Extensions>>);

impl Extensions {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(util::Extensions::new())))
    }
}

impl Deref for Extensions {
    type Target = RwLock<util::Extensions>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("QUIC error: {0}")]
    Quic(#[from] quic::Error),
}
