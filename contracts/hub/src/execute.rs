use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use steak::hub::Config;

use crate::{error::ContractError, state::CONFIG};

pub fn init(deps: DepsMut, owner: String) -> StdResult<Response> {
    CONFIG.save(
        deps.storage,
        &Config {
            owner: deps.api.addr_validate(&owner)?,
            pending_owner: None,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "steak/hub/init")
        .add_attribute("owner", owner))
}

pub fn set_owner(deps: DepsMut, new_owner: String) -> StdResult<Response> {
    let cfg = CONFIG.load(deps.storage)?;

    CONFIG.save(
        deps.storage,
        &Config {
            owner: deps.api.addr_validate(&new_owner)?,
            pending_owner: None,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "steak/hub/set_owner")
        .add_attribute("previous_owner", cfg.owner)
        .add_attribute("new_owner", new_owner))
}

pub fn transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    let mut cfg = CONFIG.load(deps.storage)?;

    if info.sender != cfg.owner {
        return Err(ContractError::NotOwner);
    }

    cfg.pending_owner = Some(deps.api.addr_validate(&new_owner)?);
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::new()
        .add_attribute("action", "steak/hub/transfer_ownership")
        .add_attribute("current_owner", cfg.owner)
        .add_attribute("proposed_owner", new_owner))
}

pub fn accept_ownership(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;

    if let Some(pending_owner) = cfg.pending_owner {
        if info.sender != pending_owner {
            return Err(ContractError::NotPendingOwner);
        }
    } else {
        return Err(ContractError::NoPendingOwnershipTransfer);
    }

    CONFIG.save(
        deps.storage,
        &Config {
            owner: info.sender.clone(),
            pending_owner: None,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "steak/hub/accept_ownership")
        .add_attribute("previous_owner", cfg.owner)
        .add_attribute("new_owner", info.sender))
}
