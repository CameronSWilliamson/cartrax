use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

/// Details required for each time gas is filled
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct GasInfo {
    /// The unique identifier for this entry
    pub id: Option<i32>,
    /// The cost of gas per gallon in USD
    pub price_per_gallon: BigDecimal,
    /// The total cost in USD
    pub total_cost: BigDecimal,
    /// The total amount of gallons purchased
    pub gallons: BigDecimal,
    /// The amount of miles on the vehicle's A Tripometer
    pub a_tripometer: BigDecimal,
    /// The amount of miles on the vehicle's B Tripometer
    pub b_tripometer: BigDecimal,
    /// The amount of miles on the vehicle's overall Tripometer
    pub total_tripometer: i32,
    /// The time this entry was created
    pub time_recorded: Option<chrono::DateTime<chrono::Utc>>,
    /// The city in which gasoline was purchased
    pub city: String,
    /// The state in which gasoline was purchased
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct GasInfoStats {
    pub total_cost: BigDecimal,
    pub total_gallons: BigDecimal,
    pub avg_ppg: BigDecimal,
    pub avg_mpg: BigDecimal,
    pub avg_a_trip: BigDecimal,
    pub avg_fill_size: BigDecimal,
}

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

/// The status of an HTTP response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseStatus {
    /// Request succeeded
    Success,
    /// Request failed
    Failure,
}
