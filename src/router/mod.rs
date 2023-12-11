use salvo::{Router, Service};
use salvo::catcher::Catcher;
use salvo::logging::Logger;
use salvo::prelude::CatchPanic;

use crate::contriller::common_controller;
use crate::server::client::login;

mod user_router;

pub fn init_router() -> Router {
    let router = Router::new()
        .hoop(Logger::new()) // 增加日志处理
        .hoop(CatchPanic::new())// 捕获处理程序中发生的 panic，并将其转换为适当的响应
        .push(user_router::init_router_no_token()) // 这里的接口不需要进行验证token
        // 下面的接口需要进行Token校验
        .push(
            Router::new().hoop(common_controller::auth_token)
                .push(
                    Router::with_path("/api")
                        .push(
                            Router::with_path("/login")
                                .post(login::login)
                        )
                        .push(
                            Router::with_path("/register")
                                .post(login::register)
                        )
                )
        );

    router
}

pub fn init_service() -> Service {
    let router = init_router();
    Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err))
}