use std::time::Duration;

use shai::{Peer, Router, local};

#[tokio::test]
async fn serve_completes() {
    let router = Router::new(());
    let local = local::Peer::new().connect(router);
    let peer = Peer::from(local);

    tokio::time::timeout(
        Duration::from_millis(50),
        async { peer.serve(Router::new(())).await.expect("local serve returns Ok") },
    )
    .await
    .expect("serve should not block on local transport");
}