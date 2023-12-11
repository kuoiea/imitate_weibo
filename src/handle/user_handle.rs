use rbatis::impl_select;

use crate::model::user_sql_model::User;
use crate::utils::config_init::GLOBAL_DB;

impl_select!(User{select_user_by_email(email:String) -> Option => "`where email = #{email} and delete_at=0  limit 1`"});
pub async fn filter_user_by_email(email: String) -> Option<User> {
    let data = User::select_user_by_email(&mut GLOBAL_DB.clone(), email).await.unwrap();
    return data;
}
