use cosmwasm_std::{Deps, StdResult};

use steak::hub::Config;

use crate::state::CONFIG;

pub fn config(deps: Deps) -> StdResult<Config<String>> {
    let cfg = CONFIG.load(deps.storage)?;
    Ok(Config {
        owner: cfg.owner.into(),
        pending_owner: cfg.pending_owner.map(String::from),
    })
}
