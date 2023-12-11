use salvo::endpoint;
use salvo::oapi::extract::JsonBody;
use uuid::Uuid;

use crate::model::user_model::{CaptchaRes, LoginReq, LoginRes, UserRegister};
use crate::model::user_sql_model::User;
use crate::utils::captcha;
use crate::utils::config_init::GLOBAL_CONFIG;
use crate::utils::jwt::create_token;
use crate::utils::redis;
use crate::utils::res::{Res, res_json_custom, res_json_err, res_json_ok, ResObj};

async fn check_captcha(captcha_uuid: String, captcha_str: String) -> Result<(), &'static str> {
    // 检查用户输入的验证码和数据库验证码是否相同
    let captcha_value = redis::get(captcha_uuid.clone()).unwrap();
    // let captcha_value: Option<String> = redis::get(captcha_uuid.clone()).unwrap_or_default();
    match captcha_value {
        Some(value) => {
            if value.is_empty() || !captcha_str.eq(&value) {
                // 如果验证码错误使用3次应该被删除， 不能多次被使用，防止暴力破解
                // 更新使用次数计数器，并判断是否超过了3次
                let count = redis::incr(&format!("usage::{:?}", captcha_uuid)).unwrap_or_default();
                if count >= 3 {
                    redis::del(captcha_uuid.clone()).unwrap();
                    redis::del(format!("usage::{:?}", captcha_uuid)).unwrap();
                }
                return Err("验证码错误");
            } else {
                // 验证成功， 删除
                redis::del(captcha_uuid.clone()).unwrap();
                return Ok(());
            }
        }
        None => {
            return Err("验证码错误");
        }
    }
}


// 生成验证码
#[endpoint(
tags("用户"),
responses(
(status_code = 200, body = ResObj < CaptchaRes >, description = "获取验证码")
),
)]
pub async fn get_captcha() -> Res<CaptchaRes> {
    // 生成图片验证码
    if let (captcha_str, Some(base64)) = captcha::create_captcha() {
        let uuid = Uuid::new_v4().to_string();
        // 验证码转小写
        redis::set_ex(&uuid, captcha_str.to_lowercase(), 300).unwrap();

        // 设置使用次数计数器，并设置过期时间为5分钟
        redis::set_ex(&format!("usage::{:?}", uuid), 0, 300).unwrap();

        Ok(res_json_ok(Some(CaptchaRes { img: base64, captcha_enabled: Some(true), uuid })))
    } else {
        Err(res_json_err("验证码生成失败".to_string()))
    }
}


/// 登录
#[endpoint(
tags("用户"),
responses(
(status_code = 200, body = ResObj < LoginRes >, description = "登录")
),
)]
pub async fn login(login_body: JsonBody<LoginReq>) -> Res<LoginRes> {
    let check_status = check_captcha(login_body.uuid.clone().unwrap(), login_body.code.clone().unwrap()).await;
    match check_status {
        Ok(()) => {
            if let (Some(email), Some(password)) = (login_body.email.clone(), login_body.password.clone()) {
                // 使用用户名
                let user_info = User::get_user_by_email(email.clone()).await;
                match user_info {
                    Some(r_op) => {
                        // 检查用户密码是否正确
                        if !User::check_password(Option::from(r_op.clone()), password).await {
                            return Err(res_json_custom(400, "用户名或密码错误".to_string()));
                        }
                        // 创建Token
                        match create_token(r_op.id.unwrap(), r_op.username.unwrap()) {
                            Ok(token) => {
                                let config = GLOBAL_CONFIG.lock().unwrap().clone().unwrap();
                                // 将token设置在redis中
                                redis::set_ex(&token, r_op.id.clone(), config.jwt_config.expiration * 60 * 60).unwrap();
                                Ok(res_json_ok(Some(LoginRes {
                                    token,
                                    expiration: (config.jwt_config.expiration * 60 * 60) as u32,
                                })))
                            }
                            Err(_err) => {
                                Err(res_json_custom(400, "token生成失败".to_string()))
                            }
                        }
                    }
                    None => Err(res_json_err("用户名或密码错误".to_string()))
                }
            } else {
                Err(res_json_custom(400, "用户账号或密码必须填写".to_string()))
            }
        }
        Err(value) => Err(res_json_err(value.to_string()))
    }
}

// 注册
#[endpoint(
tags("用户"),
responses(
(status_code = 200, body = ResObj < LoginRes >, description = "注册")
),
)]
pub async fn register(register_body: JsonBody<UserRegister>) -> Res<LoginRes> {
    let check_status = check_captcha(register_body.uuid.clone().unwrap(), register_body.code.clone().unwrap()).await;
    match check_status {
        Err(value) => Err(res_json_err(value.to_string())),
        Ok(_) => {
            // 检查密码和确认密码是否一样
            if register_body.password != register_body.confirm_password {
                Err(res_json_custom(400, "两次输入的密码不一致".to_string()))
            } else {
                // 检查用户是否存在
                if let (Some(email), Some(password), Some(user_name)) = (register_body.email.clone(), register_body.password.clone(), register_body.user_name.clone()) {
                    match User::get_user_by_email(email.clone()).await {
                        Some(_) => {
                            Err(res_json_err("用户已经存在".to_string()))
                        }
                        None => {
                            if let Ok(user) = User::create_user(user_name, password, email.clone()).await {
                                match create_token(user.id.unwrap(), user.username.unwrap()) {
                                    Ok(token) => {
                                        let config = GLOBAL_CONFIG.lock().unwrap().clone().unwrap();
                                        // 将token设置在redis中
                                        redis::set_ex(&token, user.id, config.jwt_config.expiration * 60 * 60).unwrap();
                                        Ok(res_json_ok(Some(LoginRes {
                                            token,
                                            expiration: (config.jwt_config.expiration * 60 * 60) as u32,
                                        })))
                                    }
                                    Err(_) => {
                                        Err(res_json_custom(400, "token生成失败".to_string()))
                                    }
                                }
                            } else {
                                Err(res_json_err("注册失败".to_string()))
                            }
                        }
                    }
                } else {
                    Err(res_json_custom(400, "邮箱不能为空".to_string()))
                }
            }
        }
    }
}