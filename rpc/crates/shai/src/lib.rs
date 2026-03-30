#![feature(never_type)]

mod archive;
pub mod local;
mod router;
pub mod rpc;
pub mod transport;
pub mod util;

pub use archive::Archive;
pub use router::{FromRequest, Handler, RequestContext, Router, extract};
pub use rpc::Message;
pub use transport::{Extensions, Peer};

/// Marks RPC payload types for `rkyv` wire format: adds
/// `Archive`, `Serialize`, and `Deserialize` derives (merged with any existing `#[derive]`).
/// If any `#[derive]` lists `Debug`, also adds `#[rkyv(derive(Debug))]` after the rkyv derives so
/// archived types are `Debug` in logs/traces (skipped when `#[rkyv(...)]` already names `derive` and `Debug`).
///
/// Use on a single `struct` / `enum`, or on a `mod` to apply to every struct and enum inside
/// (including nested modules). Register message IDs separately with [`rpc!`].
///
/// # Example
///
/// ```ignore
/// #[shai::message]
/// pub struct Ping;
///
/// #[shai::message]
/// pub mod farm {
///     pub struct Task { pub n: u32 }
///     pub struct Result { pub ok: bool }
/// }
///
/// shai::rpc! {
///     1: Ping => (),
///     2: farm::Task => farm::Result,
/// }
/// ```
#[cfg(feature = "macros")]
pub use shai_macros::message;

#[macro_export]
macro_rules! rpc {
    ( $( $id:literal : $req:ty => $res:ty ),* $(,)? ) => {$(
        impl $crate::rpc::Message for $req {
            const ID: $crate::rpc::MessageId = $crate::rpc::MessageId::request($id);
            type Response = $res;
        }

        impl $crate::rpc::Message for $res {
            const ID: $crate::rpc::MessageId = $crate::rpc::MessageId::response($id);
            type Response = ();
        }
    )*};
}

use std::error::Error as StdError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error - status: {status:?}, details: {payload:?}")]
    Status { status: rpc::Status, payload: bytes::Bytes },

    #[error("RPC error: {0}")]
    Rpc(#[from] rpc::Error),

    #[error("Transport error: {0}")]
    Transport(#[source] Box<dyn StdError + Send + Sync + 'static>),
}

impl Error {
    pub fn transport<E>(err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::Transport(Box::new(err))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait IntoTransport<T> {
    fn into_transport(self) -> crate::Result<T>;
}

impl<T, E> IntoTransport<T> for std::result::Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    #[inline]
    fn into_transport(self) -> crate::Result<T> {
        self.map_err(crate::Error::transport)
    }
}
