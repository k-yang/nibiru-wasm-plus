use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetExchangeRateResponse)]
    GetExchangeRate { pair: String },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetExchangeRateResponse {
    pub price: String,
}
