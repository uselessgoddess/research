use shai::extract::Extension;
use shai::rpc::{self, Status};
use shai::{Archive, Router, local};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WorkerId(pub [u8; 16]);

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct AuthReq(String);

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct AuthRes([u8; 16]);

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct MarketReq {
    pub item_id: u32,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[derive(Debug)]
#[rkyv(derive(Debug))]
pub struct MarketRes {
    pub worker: [u8; 16],
    pub item_id: u32,
}

shai::rpc! {
    10: AuthReq => AuthRes,
    11: MarketReq => MarketRes,
}

async fn handle_auth(peer: shai::Peer, req: Archive<AuthReq>) -> rpc::Result<AuthRes> {
    if req.0 == "777" {
        let token = WorkerId([0xde; 16]);
        peer.insert_extension(token);
        Ok(AuthRes(token.0))
    } else {
        Err(rpc::Error::Internal("Bad token".into()))
    }
}

async fn handle_market(
    req: Archive<MarketReq>,
    Extension(worker): Extension<WorkerId>,
) -> rpc::Result<MarketRes> {
    Ok(MarketRes { worker: worker.0, item_id: req.item_id.to_native() })
}

#[tokio::test]
async fn router_extensions_state() -> shai::Result<()> {
    let router =
        Router::new(()).route::<AuthReq, _, _>(handle_auth).route::<MarketReq, _, _>(handle_market);

    let peer_1: shai::Peer = local::Peer::new().with_id(1).connect(router.clone()).into();

    let market_req = MarketReq { item_id: 42 };

    match peer_1.call(&market_req).await.unwrap_err() {
        shai::Error::Status { status, .. } => assert_ne!(status, Status::Ok),
        _ => panic!("Expected Status error!"),
    }

    let auth_res = peer_1.call(&AuthReq("777".into())).await?;
    assert_eq!(auth_res.0, [0xde; 16]);

    let market_res = peer_1.call(&market_req).await?;
    assert_eq!(market_res.worker, [0xde; 16]);
    assert_eq!(market_res.item_id, 42);

    let peer_2: shai::Peer = local::Peer::new().with_id(2).connect(router.clone()).into();

    match peer_2.call(&market_req).await.unwrap_err() {
        shai::Error::Status { status, .. } => assert_ne!(status, Status::Ok),
        _ => panic!("Expected Status error!"),
    }

    Ok(())
}
