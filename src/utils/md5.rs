use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;


pub fn create_md5(value: String) -> Result<String, argon2::password_hash::Error> {
    // 生成一个随机的盐值
    let salt = SaltString::generate(&mut rand::thread_rng());

    // 配置 Argon2 算法参数
    let argon2 = Argon2::default();

    // 对密码进行加密
    let hash_value = argon2.hash_password(value.as_bytes(), salt.as_salt())?.to_string();

    Ok(hash_value)
}

pub fn verify_password(user_password: String, input_password: String) -> Result<bool, argon2::password_hash::Error> {
    // 验证密码是否正确
    let parsed_hash = PasswordHash::new(&user_password)?;
    let is_valid = Argon2::default().verify_password(input_password.as_bytes(), &parsed_hash).is_ok();
    Ok(is_valid)
}