use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_ownable::{Ownership, OWNERSHIP};

use steak::hub::{ExecuteMsg, InstantiateMsg, QueryMsg};

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
    cw_ownable::set_owner(deps, &msg.owner)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnership {
            new_owner,
            expiry,
        } => {
            cw_ownable::transfer_ownership(deps, &info.sender, &new_owner, expiry)?;
        }
        ExecuteMsg::AcceptOwnership {} => {
            cw_ownable::accept_ownership(deps.storage, &env.block, info.sender)?;
        }
        ExecuteMsg::RenounceOwnership {} => {
            cw_ownable::renounce_ownership(deps.storage, &info.sender)?;
        }
    }
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ownership {} => {
            let ownership = OWNERSHIP.load(deps.storage)?;
            to_binary(&Ownership {
                owner: ownership.owner.map(String::from),
                pending_owner: ownership.pending_owner.map(String::from),
                pending_expiry: ownership.pending_expiry,
            })
        },
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("{0}")]
    Ownership(#[from] cw_ownable::OwnershipError),
}
