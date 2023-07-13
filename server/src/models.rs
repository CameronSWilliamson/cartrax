use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::VersionInfo;

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

