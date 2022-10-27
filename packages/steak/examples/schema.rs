use cosmwasm_schema::write_api;

use steak::hub;

fn main() {
    write_api! {
        instantiate: hub::InstantiateMsg,
        sudo: hub::SudoMsg,
        execute: hub::ExecuteMsg,
        query: hub::QueryMsg,
    }
}
