use redis::{Commands, RedisResult, ToRedisArgs};

use crate::utils::config_init::GLOBAL_REDIS;

pub fn set<K: ToString, V: ToString>(key: K, value: V) -> RedisResult<()> {
    let client = GLOBAL_REDIS.lock().unwrap().clone().unwrap();
    let mut con = client.get_connection()?;
    con.set(key.to_string(), value.to_string())?;
    Ok(())
}

#[allow(dead_code)]
pub fn set_ex<K: ToRedisArgs, V: ToRedisArgs>(key: K, value: V, second: u64) -> RedisResult<()> {
    let client = GLOBAL_REDIS.lock().unwrap().clone().unwrap();
    let mut con = client.get_connection()?;
    let _: () = con.set_ex(key, value, second)?;

    Ok(())
}

#[allow(dead_code)]
pub fn get<K: ToRedisArgs>(key: K) -> RedisResult<Option<String>>
     {
    let client = GLOBAL_REDIS.lock().unwrap().clone().unwrap();
    let mut con = client.get_connection()?;
    let value: Option<String> = con.get(key)?;
    Ok(value)
}

#[allow(dead_code)]
#[allow(unused_must_use)]
pub fn incr<K: ToString>(key: K) -> RedisResult<i32> {
    let client = GLOBAL_REDIS.lock().unwrap().clone().unwrap();
    let mut con = client.get_connection()?;
    let result: i32 = con.incr(key.to_string(), 1)?;
    Ok(result)
}

#[allow(dead_code)]
#[allow(unused_must_use)]
pub fn del<K: ToString>(key: K) -> RedisResult<()> {
    let client = GLOBAL_REDIS.lock().unwrap().clone().unwrap();
    let mut con = client.get_connection()?;
    con.del(key.to_string())?;
    Ok(())
}