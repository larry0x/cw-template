use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use steak::hub::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};

use crate::{error::ContractError, execute, query};

pub const CONTRACT_NAME: &str = "crates.io:steak-hub";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    execute::init(deps, msg.owner)
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::SetOwner {
            new_owner,
        } => execute::set_owner(deps, new_owner),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnership {
            new_owner,
        } => execute::transfer_ownership(deps, info, new_owner),
        ExecuteMsg::AcceptOwnership {} => execute::accept_ownership(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
    }
}
