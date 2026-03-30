use std::sync::Arc;

use bytes::Bytes;
use shai::extract::State;
use shai::rpc::{Flags, Message, MessageId, Status};
use shai::{Archive, Peer, Router, local, rpc};
use tower::ServiceExt;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[derive(Debug, PartialEq)]
pub struct Ping(u64);

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[derive(Debug, PartialEq)]
pub struct Pong(u64);

rpc! {
    1: Ping => Pong,
}

async fn handle_ping(State(counter): State<u64>, req: Archive<Ping>) -> rpc::Result<Pong> {
    Ok(Pong(req.0 + counter))
}

#[tokio::test]
async fn router_ping_pong() {
    let router = Router::new(10u64).route::<Ping, _, _>(handle_ping);
    let peer: Peer = local::Peer::new().into();

    let request_data = Ping(32);
    let payload = rpc::Serialize::serialize_to_bytes(&request_data).unwrap();

    let request_frame = rpc::Frame::new(Ping::ID, Flags::EMPTY, Status::Ok, payload);
    let response_frame = router.oneshot((peer, request_frame)).await.unwrap();

    assert_eq!(response_frame.id, Pong::ID);

    let archived = <Pong as rpc::Archive>::access_bytes(&response_frame.payload).unwrap();

    assert_eq!(archived.0, 42);
}

#[tokio::test]
async fn not_found() {
    let router = Router::new(Arc::new(()));
    let peer: Peer = local::Peer::new().into();

    let ghost_frame =
        rpc::Frame::new(MessageId::request(999), Flags::EMPTY, Status::Ok, Bytes::new())
            .with_trace([1; 16]);
    let response = router.oneshot((peer, ghost_frame)).await.unwrap();

    assert_eq!(response.status, Status::NotFound);
    assert_eq!(response.trace_id, [1; 16]);
}
