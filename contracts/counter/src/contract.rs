use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
    let state = State { counter: 0 };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::IncrementCounter { by } => {
            let state = STATE.load(deps.storage)?;
            let new_counter = state
                .counter
                .checked_add(by)
                .ok_or(ContractError::Overflow {})?;

            STATE.save(
                deps.storage,
                &State {
                    counter: new_counter,
                },
            )?;
            // Increment logic here
            Ok(Response::new()
                .add_attribute("method", "increment")
                .add_attribute("by", by.to_string()))
        }
        ExecuteMsg::ResetCounter {} => {
            STATE.save(deps.storage, &State { counter: 0 })?;
            Ok(Response::new().add_attribute("method", "reset"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCounter {} => {
            let state = STATE.load(deps.storage)?;
            to_json_binary(&state.counter)
        }
    }
}
