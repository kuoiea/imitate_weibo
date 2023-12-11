use crate::model::user_model::Claims;
use crate::utils::config_init::GLOBAL_CONFIG;
use jsonwebtoken::{encode, decode, Header, Validation, errors::{Result}, EncodingKey, DecodingKey};


pub fn create_token(user_id: u64, user_name: String) -> Result<String> {
    let config = GLOBAL_CONFIG.lock().unwrap().clone().unwrap();

    let secret_key = EncodingKey::from_secret(config.jwt_config.secret.as_bytes());

    // 创建一个自定义结构体 Claims，包含需要的声明（claims）
    // JWT Token 的有效期被设置为 1 小时。
    let claims = Claims {
        user_id,
        user_name,
        sub: String::from("user123"),
        exp: (chrono::Utc::now() + chrono::Duration::hours(config.jwt_config.expiration as i64)).timestamp() as usize,
    };

    // 使用该密钥对 Claims 进行签名，生成 JWT
    let jwt_token = encode(&Header::default(), &claims, &secret_key);
    return jwt_token;
}



pub fn decode_token(jwt_token: String) {
    let config = GLOBAL_CONFIG.lock().unwrap().clone().unwrap();

    let secret_key = DecodingKey::from_secret(config.jwt_config.secret.as_bytes());
    let decoded = decode::<Claims>(&jwt_token, &secret_key, &Validation::default());
    match decoded {
        Ok(_token_data) => {
            // 处理解码成功的情况
        },
        Err(_) => {
            // 处理解码失败的情况
        }
    }

}