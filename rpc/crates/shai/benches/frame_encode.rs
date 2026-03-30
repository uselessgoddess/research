use std::hint::black_box;

use bytes::{Bytes, BytesMut};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use shai::rpc::{Flags, Frame, MessageId, Status};
use shai::transport::codec::{self, FrameCodec};
use tokio_util::codec::Encoder;

const PAYLOAD_SIZES: &[usize] = &[0, 64, 4096, 128 * 1024, 1024 * 1024];

fn encode_codec(frame: Frame, dst: &mut BytesMut) {
    let mut codec = FrameCodec::new();
    codec.encode(frame, dst).unwrap();
    dst.clear();
}

fn encode_quic(frame: Frame) -> ([u8; FrameCodec::HEADER_SIZE], Bytes) {
    let mut header = [0u8; FrameCodec::HEADER_SIZE];
    codec::encode_header(&frame, frame.payload.len(), &mut header);
    (header, frame.payload)
}

fn make_frame(payload_size: usize) -> Frame {
    let payload = Bytes::from(vec![0xABu8; payload_size]);
    Frame::new(
        MessageId::from_u16(1),
        Flags::EMPTY,
        Status::Ok,
        payload,
    )
}

fn bench_frame_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_encode");

    for &size in PAYLOAD_SIZES {
        group.throughput(if size > 0 {
            Throughput::Bytes(size as u64)
        } else {
            Throughput::Elements(1)
        });

        group.bench_with_input(
            BenchmarkId::new("codec-encode", size),
            &size,
            |b, &sz| {
                let mut dst = BytesMut::with_capacity(FrameCodec::HEADER_SIZE + sz);
                b.iter(|| {
                    let frame = make_frame(sz);
                    encode_codec(black_box(frame), &mut dst);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("quic-encode", size),
            &size,
            |b, &sz| {
                b.iter(|| {
                    let frame = make_frame(sz);
                    let (header, payload) = encode_quic(black_box(frame));
                    black_box((header, payload));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_frame_encode);
criterion_main!(benches);
