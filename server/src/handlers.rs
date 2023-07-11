use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};
use chrono::Utc;
use std::error::Error;

use crate::{
    database::{Database, self},
    models::{GasInfo, ResponseMessage, ResponseStatus, ResponseType},
};

#[get("/")]
async fn index() -> impl Responder {
    web::Json(ResponseMessage {
        status: ResponseStatus::Success,
        data: ResponseType::from("Cartrax Home Page".to_string()),
    })
}

/// Handles adding GasInfo data.
#[post("/")]
async fn post_trax_data(
    data: web::Data<Database>,
    gas_info: web::Json<GasInfo>,
) -> Result<impl Responder, Box<dyn Error>> {
    println!("Getting Data");
    let mut gas_info = gas_info.into_inner();
    if let None = gas_info.time_recorded {
        gas_info.time_recorded = Some(Utc::now());
    }
    let db = data.into_inner();
    let id = database::insert_gas_info(&db.client, &gas_info).await?;

    Ok(web::Json(ResponseMessage {
        status: ResponseStatus::Success,
        data: ResponseType::from(format!("Successfully added item with an ID of {id}")),
    }))
}

/// Handles getting GasInfo data.
#[get("/")]
async fn get_trax_data(data: web::Data<Database>) -> Result<impl Responder, Box<dyn Error>> {
    let db = data.into_inner();
    let detail_list = sqlx::query_as::<_, GasInfo>("SELECT * FROM cartrax ORDER BY id")
        .fetch_all(&db.client)
        .await?;
    Ok(web::Json(detail_list))
}

#[get("/stats/")]
async fn get_trax_stats(data: web::Data<Database>) -> Result<impl Responder, Box<dyn Error>> {
    let db = data.into_inner();
    let stats = database::get_stats(&db.client).await?;
    Ok(web::Json(stats))
}

/// Configures the `/cartrax` endpoints.
///
/// # Arguments
///
/// * `database` - The ActixWeb database to be provided to all endpoints
pub fn config(database: Database) -> impl FnOnce(&mut web::ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        let scope = web::scope("/cartrax")
            .app_data(Data::new(database))
            .service(post_trax_data)
            .service(get_trax_data)
            .service(get_trax_stats);
        cfg.service(scope);
    }
}
