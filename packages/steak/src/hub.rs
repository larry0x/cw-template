use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct Config<T> {
    /// Address of the current contract owner
    pub owner: T,
    /// If there is a pending ownership transfer, the proposed new owner
    pub pending_owner: Option<T>,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// Address of the current contract owner
    pub owner: String,
}

#[cw_serde]
pub enum SudoMsg {
    /// Forcibly reset the contract's owner.
    /// Delete the pending ownership transfer if there is one.
    SetOwner {
        new_owner: String,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Propose a transfer of the contract's ownership to another account
    TransferOwnership {
        new_owner: String,
    },

    /// Accept the proposed ownership transfer
    AcceptOwnership {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Return contract's configurations
    #[returns(Config<String>)]
    Config {},
}
