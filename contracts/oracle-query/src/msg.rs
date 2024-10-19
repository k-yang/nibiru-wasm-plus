use std::{collections::HashMap, iter::Map};

use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetExchangeRateResponse)]
    GetExchangeRate { pair: String },

    #[returns(GetExchangeRateResponse)]
    GetExchangeRateTwap { pair: String },

    #[returns(GetExchangeRatesResponse)]
    GetExchangeRates {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetExchangeRateResponse {
    pub price: String,
}

#[cw_serde]
pub struct GetExchangeRateTwapResponse {
    pub price: String,
}

#[cw_serde]
pub struct GetExchangeRatesResponse {
    // map from pair to price
    pub rates: HashMap<String, String>,
}
