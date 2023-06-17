use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GasInfo {
    pub id: Option<u32>,
    pub price_per_gallon: f32,
    pub total_price: f32,
    pub gallons: f32,
    pub trip_a: f32,
    pub trip_b: f32,
    pub mileage: i32,
    pub date: String,
    pub city: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMessage {
    pub status: ResponseStatus,
    pub data: ResponseType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseType {
    Message(String),
    GasInfo(GasInfo),
}

impl From<String> for ResponseType {
    fn from(value: String) -> Self {
        ResponseType::Message(value)
    }
}

impl From<GasInfo> for ResponseType {
    fn from(value: GasInfo) -> Self {
        ResponseType::GasInfo(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseStatus {
    Success,
    Failure,
}
