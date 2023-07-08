use std::error::Error;

use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};

use crate::{database::Database, models::*};

#[post("/")]
async fn post_trax_data(
    data: web::Data<Database<GasInfo>>,
    gas_info: web::Json<GasInfo>,
) -> Result<impl Responder, Box<dyn Error>> {
    println!("Getting Data");
    let mut gas_info = gas_info.into_inner();
    data.add_data(&mut gas_info).await?;
    let id = gas_info.id.unwrap();

    Ok(web::Json(ResponseMessage {
        status: ResponseStatus::Success,
        data: ResponseType::from(format!("Successfully added item with an ID of {id}")),
    }))
}

#[get("/")]
async fn get_trax_data(data: web::Data<Database<GasInfo>>) -> Result<impl Responder, Box<dyn Error>> {
    let detail_list = data.get_data().await?;
    Ok(web::Json(detail_list))
}

pub fn config(database: Database<GasInfo>) -> impl FnOnce(&mut web::ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        let scope = web::scope("/cartrax")
            .app_data(Data::new(database))
            .service(post_trax_data)
            .service(get_trax_data);
        cfg.service(scope);
    }
}
