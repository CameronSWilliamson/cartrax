use std::{error::Error, sync::Mutex};

use actix_web::{get, post, web, Responder};

use crate::{database::Database, models::*};

struct _AppState {
    details: Mutex<Vec<GasInfo>>,
}

#[post("/")]
async fn post_trax_data(
    data: web::Data<Database>,
    gas_info: web::Json<GasInfo>,
) -> Result<impl Responder, Box<dyn Error>> {
    println!("Getting Data");
    data.add_data(gas_info.into_inner()).await?;
    Ok(web::Json(ResponseMessage {
        status: ResponseStatus::Success,
        message: String::from("Successfully Added Item"),
    }))
}

#[get("/")]
async fn get_trax_data(data: web::Data<Database>) -> Result<impl Responder, Box<dyn Error>> {
    let detail_list = data.get_data().await?;
    Ok(web::Json(detail_list))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/cartrax")
        .service(post_trax_data)
        .service(get_trax_data);
    cfg.service(scope);
}
