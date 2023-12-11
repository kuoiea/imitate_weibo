use salvo::Router;
use crate::server::client::login;

pub fn init_router_no_token() -> Router {
    let router = Router::new();
    router.push(
        Router::with_path("/captcha-image").get(login::get_captcha)
    )

}