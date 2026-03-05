use std::net::SocketAddr;

use hyper::body::Incoming;
use jsonrpsee::core::middleware::RpcServiceBuilder;
use jsonrpsee::server::{serve_with_graceful_shutdown, stop_channel, Server, TowerServiceBuilder};
use jsonrpsee::Methods;
use tokio::net::TcpListener;
use tower::Service;
use tower_http::cors::CorsLayer;

use crate::rpc::TodoApiServer;
use crate::todo::TodoStore;

pub async fn run() {
    let store = TodoStore::new();
    let methods: Methods = store.into_rpc().into();

    let (stop_handle, server_handle) = stop_channel();

    let svc_builder = Server::builder()
        .set_http_middleware(tower::ServiceBuilder::new().layer(CorsLayer::permissive()))
        .to_service_builder();

    let listener = TcpListener::bind("127.0.0.1:3000".parse::<SocketAddr>().unwrap())
        .await
        .unwrap();

    println!("Axum + jsonrpsee server listening on 127.0.0.1:3000");
    println!("  POST /rpc    - JSON-RPC endpoint");
    println!("  GET  /health - Health check");

    // Shared state cloned per connection.
    #[derive(Clone)]
    struct PerConnection<RpcMiddleware, HttpMiddleware> {
        methods: Methods,
        stop_handle: jsonrpsee::server::StopHandle,
        svc_builder: TowerServiceBuilder<RpcMiddleware, HttpMiddleware>,
    }

    let per_conn = PerConnection {
        methods,
        stop_handle: stop_handle.clone(),
        svc_builder,
    };

    tokio::spawn(async move {
        loop {
            let sock = tokio::select! {
                res = listener.accept() => {
                    match res {
                        Ok((stream, _)) => stream,
                        Err(e) => {
                            eprintln!("accept error: {e}");
                            continue;
                        }
                    }
                }
                _ = per_conn.stop_handle.clone().shutdown() => break,
            };

            let per_conn = per_conn.clone();
            let conn_stop = per_conn.stop_handle.clone();

            let svc = tower::service_fn(move |req: hyper::Request<Incoming>| {
                let PerConnection {
                    methods,
                    stop_handle,
                    svc_builder,
                } = per_conn.clone();
                let path = req.uri().path().to_owned();
                let method = req.method().clone();

                async move {
                    if path == "/health" && method == hyper::Method::GET {
                        return Ok(hyper::Response::builder()
                            .status(200)
                            .body("OK".into())
                            .unwrap());
                    }

                    // POST /rpc: delegate to jsonrpsee tower service
                    if path == "/rpc" {
                        let rpc_middleware = RpcServiceBuilder::new();
                        let mut svc = svc_builder
                            .set_rpc_middleware(rpc_middleware)
                            .build(methods, stop_handle);
                        return svc.call(req).await;
                    }

                    // Everything else: 404
                    Ok(hyper::Response::builder()
                        .status(404)
                        .body("Not Found".into())
                        .unwrap())
                }
            });

            tokio::spawn(serve_with_graceful_shutdown(
                sock,
                svc,
                conn_stop.clone().shutdown(),
            ));
        }
    });

    server_handle.stopped().await;
}
