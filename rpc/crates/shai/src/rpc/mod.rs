mod frame;

use std::convert::Infallible;

use bytes::Bytes;
pub use frame::{Flags, MessageId, Status};

/// Message wire format (little-endian):
/// ```text
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |           Message ID          | Flags |      Status           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                        Payload Length                         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ~                    Trace ID (16 bytes, optional)              ~
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                            Payload                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug, Clone)]
pub struct Frame {
    pub id: MessageId,
    pub flags: Flags,
    pub status: Status,
    pub trace_id: [u8; 16],
    pub payload: Bytes,
}

impl Frame {
    pub fn new(id: MessageId, flags: Flags, status: Status, payload: Bytes) -> Self {
        Self { id, flags, status, trace_id: [0; 16], payload }
    }

    pub fn with_trace(mut self, trace_id: [u8; 16]) -> Self {
        self.trace_id = trace_id;
        self
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Rejection {
    #[error("Missing extension: {0}")]
    MissingExtension(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Message ID {0} not found")]
    NotFound(u16),
    #[error("Decode error")]
    Decode,
    #[error("Encode error")]
    Encode,
    #[error("Extractor rejection: {0}")]
    Reject(#[from] Rejection),
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl From<&Error> for Status {
    fn from(err: &Error) -> Self {
        match err {
            Error::NotFound(_) => Status::NotFound,
            Error::Decode => Status::DecodeError,
            Error::Encode => Status::EncodeError,
            Error::Internal(_) => Status::InternalError,
            Error::Reject(reject) => match reject {
                Rejection::MissingExtension(_) => Status::Unauthorized,
            },
        }
    }
}

pub trait Message: Archive + Serialize {
    const ID: MessageId;

    type Response: Message;
}

impl Message for () {
    const ID: MessageId = MessageId::response(0x7FFF);

    type Response = ();
}
pub trait Archive: Send + Sync + 'static {
    type Archived: 'static;

    fn access_bytes(bytes: &[u8]) -> Result<&Self::Archived, Error>;

    /// # Safety
    ///
    /// The byte slice must represent a valid archived type when accessed at the
    /// default root position. See the [rkyv docs](rkyv::api)
    unsafe fn access_unchecked(bytes: &[u8]) -> &Self::Archived;
}

#[rustfmt::skip]
impl<T> Archive for T
where
    T: rkyv::Archive + Send + Sync + 'static,
    T::Archived: for<'a> rkyv::bytecheck::CheckBytes<
        rkyv::api::high::HighValidator<'a, rkyv::rancor::Error>
    >,
{
    type Archived = T::Archived;

    #[inline]
    fn access_bytes(bytes: &[u8]) -> Result<&Self::Archived, Error> {
        rkyv::access::<Self::Archived, rkyv::rancor::Error>(bytes)
            .map_err(|_| Error::Decode)
    }

    #[inline]
    unsafe fn access_unchecked(bytes: &[u8]) -> &Self::Archived {
        unsafe { rkyv::access_unchecked(bytes) }
    }
}

pub trait Serialize: Send + Sync + 'static {
    fn serialize_to_bytes(&self) -> Result<Bytes, Error>;
}

#[rustfmt::skip]
impl<T> Serialize for T
where
    T: Send + Sync + 'static,
    T: for<'a> rkyv::Serialize<
        rkyv::api::high::HighSerializer<
            rkyv::util::AlignedVec,
            rkyv::ser::allocator::ArenaHandle<'a>,
            rkyv::rancor::Error, 
        >
    >,
{
    #[inline]
    fn serialize_to_bytes(&self) -> Result<Bytes, Error> {
        let aligned_vec =
            rkyv::to_bytes::<rkyv::rancor::Error>(self).map_err(|_| Error::Encode)?;
        // `from_owner` stores the AlignedVec inline and avoids a Vec allocation.
        Ok(Bytes::from_owner(aligned_vec))
    }
}

pub trait Deserialize<E>: Archive + Sized {
    fn deserialize(archived: &Self::Archived) -> Result<Self, E>;
}

impl<T: Archive, E> Deserialize<E> for T
where
    T::Archived: for<'a> rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<E>>,
{
    #[inline]
    fn deserialize(archived: &Self::Archived) -> Result<Self, E> {
        rkyv::deserialize(archived)
    }
}
