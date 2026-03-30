use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::time::Duration;

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use shai::extract::State;
use shai::transport::quic;
use shai::{Archive, Error, Peer, Router, rpc};
use tokio::sync::oneshot;

#[shai::message]
#[derive(Debug, PartialEq)]
pub struct FarmTask {
    node_id: u32,
    task_size: u64,
}

#[shai::message]
#[derive(Debug, PartialEq)]
pub struct FarmResult {
    processed: u64,
}

shai::rpc! {
    1: FarmTask => FarmResult,
}

async fn handle(State(multiplier): State<u64>, req: Archive<FarmTask>) -> rpc::Result<FarmResult> {
    if req.node_id == 999 {
        return Err(rpc::Error::Internal("Bad node ID".to_string()));
    }

    Ok(FarmResult { processed: req.task_size * multiplier })
}

fn generate_dummy_tls() -> (quinn::ServerConfig, quinn::ClientConfig) {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert.der().to_vec());
    let key_der = PrivateKeyDer::try_from(cert.key_pair.serialize_der()).unwrap();

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der.clone()], key_der)
        .unwrap();
    server_crypto.alpn_protocols = vec![b"shai-rpc".to_vec()];

    let quic_server_crypto =
        quinn::crypto::rustls::QuicServerConfig::try_from(server_crypto).unwrap();
    let server_config = quinn::ServerConfig::with_crypto(Arc::new(quic_server_crypto));

    let mut root_store = rustls::RootCertStore::empty();
    root_store.add(cert_der).unwrap();

    let mut client_crypto =
        rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth();
    client_crypto.alpn_protocols = vec![b"shai-rpc".to_vec()];

    let quic_client_crypto =
        quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto).unwrap();
    let client_config = quinn::ClientConfig::new(Arc::new(quic_client_crypto));

    (server_config, client_config)
}

fn server(config: quinn::ServerConfig) -> quic::Endpoint {
    quic::Endpoint::server(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)), config)
        .expect("bind server")
}

fn client(config: quinn::ClientConfig) -> quic::Endpoint {
    quic::Endpoint::client(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)), config)
        .expect("bind client")
}

fn bind_dummy() -> (quic::Endpoint, quic::Endpoint) {
    let (server_config, client_config) = generate_dummy_tls();
    (server(server_config), client(client_config))
}

#[tokio::test]
async fn quic_e2e_rpc() {
    let (server, client) = bind_dummy();
    let bound_addr = server.local_addr().unwrap();

    let router = Router::new(10u64).route::<FarmTask, _, _>(handle);
    tokio::spawn(async move {
        server.serve(router).await;
    });

    let peer =
        client.connect(bound_addr, "localhost").await.map(Peer::from).expect("Failed to connect");

    let request = FarmTask { node_id: 1, task_size: 5 };
    let trace_id = [7u8; 16];

    let response = peer.request_raw(&request, trace_id).await.unwrap();
    assert_eq!(response.processed, 50);

    let bad_request = FarmTask { node_id: 999, task_size: 5 };
    let bad_response = peer.call(&bad_request).await;

    match bad_response {
        Err(Error::Status { status, payload }) => {
            assert_eq!(status, rpc::Status::InternalError);
            assert_eq!(payload.as_ref(), b"Bad node ID");
        }
        _ => panic!("Expected RPC Internal Error"),
    }
}

#[tokio::test]
async fn endpoint_accept_incoming() {
    let (server, client) = bind_dummy();
    let bound_addr = server.local_addr().unwrap();

    let router = Router::new(10u64).route::<FarmTask, _, _>(handle);
    let (ready_tx, ready_rx) = oneshot::channel();

    let server_task = tokio::spawn(async move {
        let peer = match server.accept().await {
            Some(Ok(p)) => Peer::from(p),
            Some(Err(e)) => panic!("handshake: {e}"),
            None => panic!("endpoint closed before accept"),
        };
        ready_tx.send(()).ok();
        let _ = peer.serve(router).await;
    });

    let peer =
        client.connect(bound_addr, "localhost").await.map(Peer::from).expect("Failed to connect");
    ready_rx.await.expect("server accepted");

    let response = peer.call(&FarmTask { node_id: 2, task_size: 3 }).await.unwrap();
    assert_eq!(response.processed, 30);

    drop(peer);
    drop(client);

    tokio::time::timeout(Duration::from_secs(5), server_task)
        .await
        .expect("server task timed out")
        .expect("server task panicked");
}

#[tokio::test]
async fn peer_ends_when_connection_drops() {
    let (server, client) = bind_dummy();
    let bound_addr = server.local_addr().unwrap();

    let router = Router::new(10u64).route::<FarmTask, _, _>(handle);

    let server_task = tokio::spawn(async move {
        let peer = match server.accept().await {
            Some(Ok(p)) => Peer::from(p),
            _other => panic!("unexpected accept"),
        };
        let _ = peer.serve(router).await;
    });

    let peer =
        client.connect(bound_addr, "localhost").await.map(Peer::from).expect("Failed to connect");

    assert_eq!(peer.call(&FarmTask { node_id: 1, task_size: 4 }).await.unwrap().processed, 40);

    drop(peer);
    drop(client);

    tokio::time::timeout(Duration::from_secs(5), server_task)
        .await
        .expect("server task timed out")
        .expect("server task panicked");
}
