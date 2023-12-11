use salvo::{FlowCtrl, handler, Depot,prelude::StatusCode, Response, Request};

use crate::utils::redis;
use crate::utils::res::res_json_custom;

#[handler]
pub async fn catcher_err(res: &mut Response, ctrl: &mut FlowCtrl) {
    println!("发生panic");
    if let Some(status_code) = res.status_code {
        match status_code {
            StatusCode::NOT_FOUND=>{
                res.render(res_json_custom::<()>(404, "没有对应的接口".to_string()));
            },
            StatusCode::INTERNAL_SERVER_ERROR=>{
                ctrl.skip_rest();
                res.body("".into());
                res.render(res_json_custom::<()>(500, "服务器发生错误".to_string()));
            },
            _=>{
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.body("".into());
                res.render(res_json_custom::<()>(500, "服务器发生错误".to_string()));
                ctrl.skip_rest();
            }
        }
    }
}

#[handler]
pub async fn auth_token(req:&mut Request,res:&mut Response, ctrl: &mut FlowCtrl,depot: &mut Depot){
    if let Some(token) = req.headers().get("Authorization"){
        // 验证token

    }else{
        ctrl.skip_rest();
        res.render(res_json_custom::<()>(401,"token无效".to_string()));
    }

}