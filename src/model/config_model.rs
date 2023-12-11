use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MysqlConfigInfoStruct {
    pub username: Box<str>,
    pub password: Box<str>,
    pub host: Box<str>,
    pub port: Box<str>,
    pub database_name: Box<str>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MysqlConfigDictStrut {
    pub default: MysqlConfigInfoStruct,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RedisConfigStruct {
    pub host: Box<str>,
    pub port: Box<str>,
    pub password: Box<str>,
    pub db: Box<str>,
}

#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct JwtConfigStruct {
    pub jwt_secret: Box<str>,
    pub expiration: u64,
}

#[derive(Debug, Clone,Deserialize)]
pub struct Config {
    pub database: MysqlConfigDictStrut,
    pub redis: RedisConfigStruct,
    pub jwt_config: JwtConfigStruct,
}