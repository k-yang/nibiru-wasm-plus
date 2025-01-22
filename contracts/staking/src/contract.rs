use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo,
    Response, StakingMsg, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:infinite-loop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Run {} => {
            let msg: CosmosMsg<Empty> = CosmosMsg::Staking(StakingMsg::Delegate {
                validator: "nibivaloper1zaavvzxez0elundtn32qnk9lkm8kmcszuwx9jz".to_string(), // guard creamer's valoper address
                amount: info.funds[0].clone(),
            });

            Ok(Response::new()
                .add_message(msg)
                .add_attribute("method", "run")
                .add_attribute(
                    "validator",
                    "nibivaloper1zaavvzxez0elundtn32qnk9lkm8kmcszuwx9jz".to_string(),
                )
                .add_attribute("amount", info.funds[0].amount.to_string()))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}
