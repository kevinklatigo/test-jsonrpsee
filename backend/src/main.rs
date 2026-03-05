mod auth;
mod rpc;
mod todo;

#[cfg(feature = "standalone")]
mod standalone;
#[cfg(feature = "axum")]
mod axum_server;

#[tokio::main]
async fn main() {
    #[cfg(feature = "standalone")]
    standalone::run().await;

    #[cfg(feature = "axum")]
    axum_server::run().await;
}
