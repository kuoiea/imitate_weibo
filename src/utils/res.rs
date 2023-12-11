use salvo::{oapi::{ToSchema}};
use salvo::prelude::Json;

pub use crate::model::common_model::ResObj;


impl<T:ToSchema> ResObj<T> {
    pub fn ok(data: Option<T>)->Self{
        Self {
            code: 0,
            msg: "访问成功".to_string(),
            data,
        }
    }
    pub fn custom_code(code:i32,msg:String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }

    pub fn err(err:String)->Self{
        Self {
            code: 500,
            msg: err,
            data: None,
        }
    }
}

#[allow(dead_code)]
pub fn res_json_ok<T:ToSchema>(data:Option<T>)->Json<ResObj<T>>{
    Json(ResObj::ok(data))
}

#[allow(dead_code)]
pub fn res_json_err<T:ToSchema>(msg:String)->Json<ResObj<T>>{
    Json(ResObj::err(msg))
}


#[allow(dead_code)]
pub fn res_json_custom<T:ToSchema>(code:i32,msg:String)->Json<ResObj<T>>{
    Json(ResObj::custom_code(code,msg))
}

#[allow(dead_code)]
pub type Res<T> = Result<Json<ResObj<T>>,Json<ResObj<()>>>;
