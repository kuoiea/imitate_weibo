use chrono::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::handle::user_handle::filter_user_by_email;
use crate::utils::md5::{create_md5, verify_password};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Queryable)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub avatar: Option<String>,
    pub(crate) create_time: u32,
    pub(crate) delete_at: u32,
}


impl User {
    pub async fn create_user(user_name: String, password: String, email: String) -> Result<User, &'static str> {
        let password = create_md5(password);
        let hash_password:String;
        match password {
            Ok(value) => {
                hash_password = value
            },
            Err(_) => {
                return Err("密码加密失败")
            }
        }

        let new_user = User {
            id: None,
            username: Some(user_name),
            password: Some(hash_password),
            email: Some(email),
            status: Some(String::from("USING")),
            avatar: Some(String::from("default.jpg")),
            create_time: Utc::now().timestamp() as u32,
            delete_at: 0,
        };

        // 需要插入数据
        // let _ = User::insert(&mut GLOBAL_DB.clone(), &new_user).await.unwrap();

        return Ok(new_user);
    }

    pub(crate) async fn get_user_by_email(email: String) -> Option<User> {
        return filter_user_by_email(email).await;
    }

    pub(crate) async fn check_password(user: Option<User>, password: String) -> bool {
        match user {
            Some(user) => {
                if let Ok(is_ok) = verify_password(user.password.unwrap(), password) {
                    match is_ok {
                        true => true,
                        false => false,

                    }
                } else {
                    // 否则返回None
                    false
                }
            }
            None => false
        }
    }
}


