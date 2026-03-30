mod peer;

pub use peer::{Endpoint, Peer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("QUIC connect error: {0}")]
    Connect(#[from] quinn::ConnectError),

    #[error("QUIC connection error: {0}")]
    Connection(#[from] quinn::ConnectionError),

    #[error("QUIC stream write error: {0}")]
    Write(#[from] quinn::WriteError),

    #[error("QUIC stream read error: {0}")]
    Read(#[from] quinn::ReadError),

    #[error("QUIC close stream: {0}")]
    Closed(#[from] quinn::ClosedStream),

    #[error("Payload too large: {0}")]
    PayloadTooLarge(usize),

    #[error("Stream closed unexpectedly before reading response")]
    UnexpectedEnd,
}
