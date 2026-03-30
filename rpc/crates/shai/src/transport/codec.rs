use std::io;

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::rpc::{Flags, Frame, MessageId, Status};

/// Both [`FrameCodec::encode`] and the zero-copy QUIC writer use this to
/// guarantee a single source of truth for field order and endianness.
pub fn encode_header(frame: &Frame, payload_len: usize, out: &mut [u8; FrameCodec::HEADER_SIZE]) {
    out[0..2].copy_from_slice(&frame.id.as_u16().to_le_bytes());
    out[2] = frame.flags.as_u8();
    out[3] = frame.status.as_u8();
    out[4..8].copy_from_slice(&(payload_len as u32).to_le_bytes());
    out[8..24].copy_from_slice(&frame.trace_id);
}

pub struct FrameCodec {
    max_frame_size: usize,
}

impl FrameCodec {
    /// ID (2) + Flags (1) + Status (1) + Length (4) + TraceID (16) = 24.
    pub const HEADER_SIZE: usize = 24;
    pub const DEFAULT_MAX_SIZE: usize = 16 * 1024 * 1024;
    pub const DEFAULT_BUFFER_CAPACITY: usize = 256 * 1024;

    pub fn new() -> Self {
        Self { max_frame_size: Self::DEFAULT_MAX_SIZE }
    }

    pub fn with_max_size(max_size: usize) -> Self {
        Self { max_frame_size: max_size }
    }
}

impl Default for FrameCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder<Frame> for FrameCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let payload_len = item.payload.len();

        if payload_len > self.max_frame_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Payload size {} exceeds max frame size {}",
                    payload_len, self.max_frame_size
                ),
            ));
        }

        dst.reserve(Self::HEADER_SIZE + payload_len);

        let mut header = [0u8; Self::HEADER_SIZE];
        encode_header(&item, payload_len, &mut header);
        dst.put_slice(&header);
        dst.put(item.payload);

        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < Self::HEADER_SIZE {
            return Ok(None);
        }

        let len_bytes: [u8; 4] = src[4..8].try_into().unwrap();
        let payload_len = u32::from_le_bytes(len_bytes) as usize;

        if payload_len > self.max_frame_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Frame too large: {} > {}", payload_len, self.max_frame_size),
            ));
        }

        let frame_len = Self::HEADER_SIZE + payload_len;

        if src.len() < frame_len {
            src.reserve(frame_len - src.len());
            return Ok(None);
        }

        let mut frame_data = src.split_to(frame_len);

        let id = frame_data.get_u16_le();
        let flags = frame_data.get_u8();
        let status = frame_data.get_u8();
        let _len = frame_data.get_u32_le();

        let mut trace_id = [0u8; 16];
        frame_data.copy_to_slice(&mut trace_id);

        Ok(Some(Frame {
            id: MessageId::from_u16(id),
            flags: Flags::from_u8(flags),
            status: Status::from_u8(status),
            trace_id,
            payload: frame_data.freeze(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn codec_encode_decode() {
        let mut codec = FrameCodec::new();
        let mut buffer = BytesMut::new();

        let original_frame = Frame {
            id: MessageId::from_u16(42),
            flags: Flags::from_u8(1),
            status: Status::NotFound,
            trace_id: [7; 16],
            payload: Bytes::from_static(b"hello world"),
        };
        codec.encode(original_frame.clone(), &mut buffer).unwrap();

        assert_eq!(buffer.len(), 24 + 11);

        let decoded_frame = codec.decode(&mut buffer).unwrap().expect("Frame should be complete");

        assert_eq!(decoded_frame.id.as_u16(), 42);
        assert_eq!(decoded_frame.flags.as_u8(), 1);
        assert_eq!(decoded_frame.status, Status::NotFound);
        assert_eq!(decoded_frame.trace_id, [7; 16]);
        assert_eq!(decoded_frame.payload.as_ref(), b"hello world");

        assert!(buffer.is_empty());
    }

    #[test]
    fn codec_fragmented_decode() {
        let mut codec = FrameCodec::new();
        let mut buffer = BytesMut::new();

        let original_frame = Frame {
            id: MessageId::from_u16(100),
            flags: Flags::from_u8(0),
            status: Status::Ok,
            trace_id: [0; 16],
            payload: Bytes::from_static(b"chunked payload"),
        };

        let mut temp = BytesMut::new();
        codec.encode(original_frame, &mut temp).unwrap();

        buffer.extend_from_slice(&temp[0..10]);
        let res = codec.decode(&mut buffer).unwrap();
        assert!(res.is_none(), "Should wait for more bytes");

        buffer.extend_from_slice(&temp[10..24]);
        let res = codec.decode(&mut buffer).unwrap();
        assert!(res.is_none(), "Should wait for payload bytes");

        buffer.extend_from_slice(&temp[24..]);
        let decoded_frame = codec.decode(&mut buffer).unwrap().unwrap();

        assert_eq!(decoded_frame.payload.as_ref(), b"chunked payload");
    }
}
