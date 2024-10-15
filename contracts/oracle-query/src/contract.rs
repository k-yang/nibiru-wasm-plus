use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use nibiru_std::proto::{
    nibiru::oracle::{QueryExchangeRateRequest, QueryExchangeRateResponse},
    NibiruProstMsg, NibiruStargateQuery,
};
use prost::Message;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetExchangeRateResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle-query";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetExchangeRate { pair } => {
            to_json_binary(&query::query_exchange_rate(deps, pair)?)
        }
    }
}

pub mod query {
    // use cosmwasm_std::GrpcQuery;

    use super::*;

    pub fn query_exchange_rate(deps: Deps, pair: String) -> StdResult<GetExchangeRateResponse> {
        let query = QueryExchangeRateRequest { pair };

        let res: Binary = deps.querier.query_grpc(query.path(), query.to_binary())?;
        let query_resp = QueryExchangeRateResponse::decode(res.as_slice());

        Ok(GetExchangeRateResponse {
            price: query_resp.unwrap().exchange_rate,
        })
    }
}
