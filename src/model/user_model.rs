use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};


// 验证码返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[salvo(schema(rename_all = "camelCase"))]
pub struct CaptchaRes {
    #[serde(rename = "captchaEnabled")]
    pub captcha_enabled: Option<bool>,
    pub img: String,
    pub uuid: String,
}


// 登录请求参数
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct LoginReq {
    pub code: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub uuid: Option<String>, // 在用户进入登陆界面的时候， uuid和code应该一起返回给客户端
}


// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginRes {
    pub token: String,
    pub expiration: u32,
}

// 生成JWT token使用
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u64,
    pub user_name: String,
    pub(crate) sub: String,
    pub(crate) exp: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct UserRegister{
    pub code: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
    pub email: Option<String>,
    pub uuid: Option<String>,
    pub user_name: Option<String>,
}

