use crate::{database::Database, models::*};
use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};
use chrono::Utc;
use std::error::Error;

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
    let database = data.into_inner();
    let client = database.client.lock().unwrap();
    let id = client.insert_gas_info(&gas_info).await?;

    Ok(web::Json(ResponseMessage {
        status: ResponseStatus::Success,
        data: ResponseType::from(format!("Successfully added item with an ID of {id}")),
    }))
}

/// Handles getting GasInfo data.
#[get("/")]
async fn get_trax_data(data: web::Data<Database>) -> Result<impl Responder, Box<dyn Error>> {
    let database = data.into_inner();
    let client = database.client.lock().unwrap();
    let detail_list = sqlx::query_as::<_, GasInfo>("SELECT * FROM cartrax ORDER BY id")
        .fetch_all(&client.pg)
        .await?;
    Ok(web::Json(detail_list))
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
            .service(get_trax_data);
        cfg.service(scope);
    }
}
