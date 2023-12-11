use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use redis::{Client, RedisResult};

use crate::model::config_model::Config;

pub async fn init_config() {
    // 打开配置文件
    let mut file = File::open("src/config/config.json")
        .expect("Failed to open config file");

    // 读取配置文件内容到字符串
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config file");

    // 解析配置文件为 Config 结构体
    let config: Config = serde_json::from_str(&contents).expect("Failed to parse config file");

    *GLOBAL_CONFIG.lock().unwrap() = Some(config);

    init_mysql().await;
    init_redis().await.expect("Failed to initialize Redis");
}

// 初始化 Mysql
async fn init_mysql() {
    let config = GLOBAL_CONFIG.lock().unwrap().clone().unwrap();
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.database.default.username,
        config.database.default.password,
        config.database.default.host,
        config.database.default.port,
        config.database.default.database_name,
    ).to_string();

    GLOBAL_DB.init(
        MysqlDriver {},
        &*url,
    )
        .expect("数据库连接失败");
}

// 初始化 Redis
async fn init_redis() -> RedisResult<()> {
    let config = GLOBAL_CONFIG.lock().unwrap().clone().unwrap();

    let redis_url = format!("redis://{}:{}/", config.redis.host, config.redis.port);
    let client = Client::open(redis_url)?;
    let mut con = client.get_connection()?;
    // 对连接进行认证
    redis::cmd("AUTH")
        .arg(format!("{}", config.redis.password))
        .query(&mut con)?;

    *GLOBAL_REDIS.lock().unwrap() = Some(client);
    Ok(())
}

pub static GLOBAL_DB: Lazy<RBatis> = Lazy::new(|| RBatis::new());
pub(crate) static GLOBAL_REDIS: Lazy<Arc<Mutex<Option<Client>>>> =  Lazy::new(|| Arc::new(Mutex::new(None)));
pub static GLOBAL_CONFIG: Lazy<Arc<Mutex<Option<Config>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));