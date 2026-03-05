use http::Method;
use jsonrpsee::server::Server;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::auth::AuthLayer;
use crate::rpc::TodoApiServer;
use crate::todo::TodoStore;

pub async fn run() {
    let store = TodoStore::new();
    let module = store.into_rpc();

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]);

    let middleware = tower::ServiceBuilder::new()
        .layer(cors)
        .layer(AuthLayer::bearer("super-secret-token"));

    let server = Server::builder()
        .set_http_middleware(middleware)
        .build("127.0.0.1:3000".parse::<SocketAddr>().unwrap())
        .await
        .unwrap();

    let addr = server.local_addr().unwrap();
    println!("Standalone jsonrpsee server listening on {addr}");

    let handle = server.start(module);
    handle.stopped().await;
}
