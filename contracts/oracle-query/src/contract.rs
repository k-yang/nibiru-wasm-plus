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
        QueryMsg::GetExchangeRateTwap { pair } => {
            to_json_binary(&query::query_exchange_rate_twap(deps, pair)?)
        }
        QueryMsg::GetExchangeRates {} => to_json_binary(&query::query_exchange_rates(deps)?),
    }
}

pub mod query {
    use std::{collections::HashMap, hash::Hash};

    use nibiru_std::proto::nibiru::oracle::{
        QueryExchangeRatesRequest, QueryExchangeRatesResponse,
    };
    use prost::Name;

    use crate::msg::GetExchangeRatesResponse;

    use super::*;

    pub fn query_exchange_rate(deps: Deps, pair: String) -> StdResult<GetExchangeRateResponse> {
        let query = QueryExchangeRateRequest { pair };

        let res: Binary = deps.querier.query_grpc(query.path(), query.to_binary())?;
        let query_resp = QueryExchangeRateResponse::decode(res.as_slice());

        Ok(GetExchangeRateResponse {
            price: query_resp.unwrap().exchange_rate,
        })
    }

    pub fn query_exchange_rate_twap(
        deps: Deps,
        pair: String,
    ) -> StdResult<GetExchangeRateResponse> {
        let query = QueryExchangeRateRequest { pair };

        let res: Binary = deps.querier.query_grpc(
            String::from(format!(
                "/{}.{}",
                QueryExchangeRateRequest::PACKAGE,
                "Query/ExchangeRateTwap",
            )),
            query.to_binary(),
        )?;
        let query_resp = QueryExchangeRateResponse::decode(res.as_slice());

        Ok(GetExchangeRateResponse {
            price: query_resp.unwrap().exchange_rate,
        })
    }

    pub fn query_exchange_rates(deps: Deps) -> StdResult<GetExchangeRatesResponse> {
        let query = QueryExchangeRatesRequest {};

        let res: Binary = deps.querier.query_grpc(query.path(), query.to_binary())?;
        let query_resp = QueryExchangeRatesResponse::decode(res.as_slice());

        let mut resp = GetExchangeRatesResponse {
            rates: HashMap::new(),
        };

        for exchange_rate_tuple in query_resp.unwrap().exchange_rates {
            resp.rates
                .insert(exchange_rate_tuple.pair, exchange_rate_tuple.exchange_rate);
        }

        Ok(resp)
    }
}
