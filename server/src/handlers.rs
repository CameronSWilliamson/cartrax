use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{
    database::{self, Database},
    models::GasInfo,
    VersionInfo,
};

/// The structure for every HTTP response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMessage {
    /// The status of the response
    pub status: ResponseStatus,
    /// The data included in the response
    pub data: ResponseType,
}

/// The type of data included in a ResponseMessage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseType {
    /// A response that contains a string
    Message(String),
    /// A response that contains information on gasoline
    GasInfo(Box<GasInfo>),
    /// A response that contains version info
    Version(Box<VersionInfo>),
}

impl From<String> for ResponseType {
    fn from(value: String) -> Self {
        ResponseType::Message(value)
    }
}

impl From<GasInfo> for ResponseType {
    fn from(value: GasInfo) -> Self {
        ResponseType::GasInfo(Box::new(value))
    }
}

impl From<VersionInfo> for ResponseType {
    fn from(value: VersionInfo) -> Self {
        ResponseType::Version(Box::new(value))
    }
}

/// The status of an HTTP response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseStatus {
    /// Request succeeded
    Success,
    /// Request failed
    Failure,
}

#[get("/version")]
async fn version(data: web::Data<VersionInfo>) -> impl Responder {
    web::Json(data.into_inner())
}

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
    let mut gas_info = gas_info.into_inner();
    if gas_info.time_recorded.is_none() {
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
