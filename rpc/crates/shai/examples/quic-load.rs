use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::time::Instant;

use quinn::VarInt;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use shai::transport::quic::Endpoint;
use shai::{Archive, Peer, Router, rpc};

#[shai::message]
pub struct SmallReq([f32; 3]);

#[shai::message]
pub struct SmallRes(bool);

#[shai::message]
pub struct LargeReq(Vec<u8>);

#[shai::message]
pub struct LargeRes(u32);

shai::rpc! {
    1: SmallReq => SmallRes,
    2: LargeReq => LargeRes,
}

async fn handle_small(_req: Archive<SmallReq>) -> rpc::Result<SmallRes> {
    Ok(SmallRes(true))
}

async fn handle_large(req: Archive<LargeReq>) -> rpc::Result<LargeRes> {
    Ok(LargeRes(req.0.len() as u32))
}

fn generate_highload_tls() -> (quinn::ServerConfig, quinn::ClientConfig) {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert.der().to_vec());
    let key_der = PrivateKeyDer::try_from(cert.key_pair.serialize_der()).unwrap();

    let mut transport_config = quinn::TransportConfig::default();

    transport_config.max_concurrent_bidi_streams(100_000u32.into());
    transport_config.stream_receive_window(VarInt::from_u32(16 * 1024 * 1024u32));
    transport_config.receive_window(VarInt::from_u32(64 * 1024 * 1024u32));
    transport_config.max_idle_timeout(None);
    // Raise the local send-window cap so high concurrency does not stall
    // waiting for the unacked-data limit to clear.
    transport_config.send_window(256 * 1024 * 1024);

    let transport_config = Arc::new(transport_config);

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der.clone()], key_der)
        .unwrap();
    server_crypto.alpn_protocols = vec![b"shai-rpc".to_vec()];

    let quic_server_crypto =
        quinn::crypto::rustls::QuicServerConfig::try_from(server_crypto).unwrap();
    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(quic_server_crypto));
    server_config.transport_config(transport_config.clone());

    let mut root_store = rustls::RootCertStore::empty();
    root_store.add(cert_der).unwrap();

    let mut client_crypto =
        rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth();
    client_crypto.alpn_protocols = vec![b"shai-rpc".to_vec()];

    let quic_client_crypto =
        quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto).unwrap();
    let mut client_config = quinn::ClientConfig::new(Arc::new(quic_client_crypto));
    client_config.transport_config(transport_config);

    (server_config, client_config)
}

#[tokio::main]
async fn main() {
    let (server_config, client_config) = generate_highload_tls();

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0));
    let server = Endpoint::server(addr, server_config).unwrap();
    let bound_addr = server.local_addr().unwrap();

    let router =
        Router::new(()).route::<SmallReq, _, _>(handle_small).route::<LargeReq, _, _>(handle_large);

    tokio::spawn(async move {
        server.serve(router).await;
    });

    let client_endpoint =
        Endpoint::client(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)), client_config)
            .unwrap();

    let peer = Peer::from(client_endpoint.connect(bound_addr, "localhost").await.unwrap());

    println!("\nStarting SHAI framework stress test...\n");

    let concurrent_workers = 500;
    let requests_per_worker = 100;
    let total_small_requests = concurrent_workers * requests_per_worker;

    let start = Instant::now();
    let mut tasks = Vec::with_capacity(concurrent_workers);

    for _ in 0..concurrent_workers {
        let peer = peer.clone();
        tasks.push(tokio::spawn(async move {
            for _ in 0..requests_per_worker {
                let req = SmallReq([1.0, 2.0, 3.0]);
                let _res = peer.call(&req).await.expect("Small call failed");
            }
        }));
    }

    futures::future::join_all(tasks).await;
    let elapsed = start.elapsed();
    let rps = total_small_requests as f64 / elapsed.as_secs_f64();

    println!(
        "Successfully processed {} requests (across {} workers).",
        total_small_requests, concurrent_workers
    );
    println!(" Time: {:?}", elapsed);
    println!(" RPS: {:.2} req/sec\n", rps);

    let payload_size = 300 * 1024;
    let total_large_requests = 1000;

    let large_data = vec![42u8; payload_size];

    let start = Instant::now();
    let mut tasks = Vec::with_capacity(total_large_requests);

    for _ in 0..total_large_requests {
        let peer = peer.clone();
        let data = large_data.clone();
        tasks.push(tokio::spawn(async move {
            let req = LargeReq(data);
            let _ = peer.call(&req).await.expect("Large call failed");
        }));
    }

    futures::future::join_all(tasks).await;
    let elapsed = start.elapsed();
    let total_mb = (payload_size * total_large_requests) as f64 / (1024.0 * 1024.0);
    let mbps = total_mb / elapsed.as_secs_f64();

    println!(
        "Successfully transferred {} files of {} KB each.",
        total_large_requests,
        payload_size / 1024
    );
    println!(" Time: {:?}", elapsed);
    println!(" Total Data Transferred: {:.2} MB", total_mb);
    println!(" Speed: {:.2} MB/sec ( {:.2} Mbps )", mbps, mbps * 8.0);
}
