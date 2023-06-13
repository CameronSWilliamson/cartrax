use std::{ops::Deref, sync::Mutex};

use actix_web::{get, post, web, Responder};

use crate::models::*;

struct AppState {
    details: Mutex<Vec<GasInfo>>,
}

#[post("/")]
async fn post_trax_data(
    data: web::Data<AppState>,
    gas_info: web::Json<GasInfo>,
) -> actix_web::Result<impl Responder> {
    let mut detail_list = data.details.lock().unwrap();
    detail_list.push(gas_info.into_inner());
    Ok(web::Json(ResponseMessage {
        status: ResponseStatus::Success,
        message: String::from("Successfully Added Item"),
    }))
}

#[get("/")]
async fn get_trax_data(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let detail_list = data.details.lock().unwrap();
    Ok(web::Json(detail_list.deref().clone()))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/cartrax")
        .app_data(web::Data::new(AppState {
            details: Mutex::new(Vec::new()),
        }))
        .service(post_trax_data)
        .service(get_trax_data);
    cfg.service(scope);
}
