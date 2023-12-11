use salvo::prelude::*;
use crate::utils::config_init::init_config;

mod router;
mod server;
mod utils;
mod model;
mod contriller;
mod handle;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // 初始化配置项
    init_config().await;

    let service = router::init_service();

    Server::new(
        TcpListener::new("0.0.0.0:8090").bind().await
    ).serve(service).await;
}