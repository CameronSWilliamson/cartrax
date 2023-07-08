use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct GasInfo {
    pub id: Option<i32>,
    pub price_per_gallon: BigDecimal,
    pub total_cost: BigDecimal,
    pub gallons: BigDecimal,
    pub a_tripometer: BigDecimal,
    pub b_tripometer: BigDecimal,
    pub total_tripometer: i32,
    pub time_recorded: Option<chrono::DateTime<chrono::Utc>>,
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
