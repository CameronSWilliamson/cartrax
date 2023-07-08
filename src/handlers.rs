use std::error::Error;

use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};
use chrono::Utc;

use crate::{database::Database, models::*};

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
    let id: (i32,) = sqlx::query_as(
        "INSERT INTO cartrax 
        (price_per_gallon, total_cost, gallons, a_tripometer,
         b_tripometer, total_tripometer, time_recorded, city, state)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id;",
    )
    .bind(gas_info.price_per_gallon)
    .bind(gas_info.total_cost)
    .bind(gas_info.gallons)
    .bind(gas_info.a_tripometer)
    .bind(gas_info.b_tripometer)
    .bind(gas_info.total_tripometer)
    .bind(gas_info.time_recorded)
    .bind(gas_info.city)
    .bind(gas_info.state)
    .fetch_one(&client.pg)
    .await?;

    //let id = gas_info.id.unwrap();
    let id = id.0;

    Ok(web::Json(ResponseMessage {
        status: ResponseStatus::Success,
        data: ResponseType::from(format!("Successfully added item with an ID of {id}")),
    }))
}

#[get("/")]
async fn get_trax_data(data: web::Data<Database>) -> Result<impl Responder, Box<dyn Error>> {
    let database = data.into_inner();
    let client = database.client.lock().unwrap();
    let detail_list = sqlx::query_as::<_, GasInfo>("SELECT * FROM cartrax ORDER BY id")
        .fetch_all(&client.pg)
        .await?;
    Ok(web::Json(detail_list))
}

pub fn config(database: Database) -> impl FnOnce(&mut web::ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        let scope = web::scope("/cartrax")
            .app_data(Data::new(database))
            .service(post_trax_data)
            .service(get_trax_data);
        cfg.service(scope);
    }
}
