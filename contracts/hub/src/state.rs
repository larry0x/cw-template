use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use steak::hub::Config;

pub const CONFIG: Item<Config<Addr>> = Item::new("cfg");
