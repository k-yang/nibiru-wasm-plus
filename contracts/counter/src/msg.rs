
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    IncrementCounter { by: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetCounterResponse)]
    GetCounter {},
}

#[cw_serde]
pub struct GetCounterResponse {
    pub counter: u64,
}
