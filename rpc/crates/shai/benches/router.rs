use std::hint::black_box;
use std::mem::size_of;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use shai::extract::*;
use shai::rpc::{self, Flags, Frame, Message, Status};
use shai::{Archive, Peer, Router, local};
use tower::{Service, ServiceBuilder, ServiceExt};

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct SmallRequest {
    pub id: u64,
    pub data: [u8; 32],
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct SmallResponse {
    pub result: u64,
}

shai::rpc! {
    1: SmallRequest => SmallResponse
}

async fn bench_handler(
    State(state): State<u64>,
    req: Archive<SmallRequest>,
) -> rpc::Result<SmallResponse> {
    Ok(SmallResponse { result: req.id + state })
}

fn bench_router_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

    let router = Router::new(100u64).route::<SmallRequest, _, _>(bench_handler);
    let app = ServiceBuilder::new().concurrency_limit(1024).service(router);

    let payload =
        rpc::Serialize::serialize_to_bytes(&SmallRequest { id: 42, data: [1u8; 32] }).unwrap();
    let frame = Frame::new(SmallRequest::ID, Flags::EMPTY, Status::Ok, payload);
    let peer: Peer = local::Peer::new().into();

    let mut group = c.benchmark_group("RPC Router");
    group.throughput(Throughput::ElementsAndBytes {
        elements: 1,
        bytes: size_of::<SmallRequest>() as u64,
    });

    group.bench_function("zero_copy_unary", |b| {
        b.to_async(&rt).iter(|| {
            let mut svc = app.clone();
            let input = frame.clone();
            let peer = peer.clone();

            async move {
                let response = svc.ready().await.unwrap().call((peer, input)).await.unwrap();
                black_box(response);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_router_throughput);
criterion_main!(benches);
