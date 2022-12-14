/// Message type definitions of the Steak Hub contract
pub mod hub {
    use cosmwasm_schema::{cw_serde, QueryResponses};
    use cw_ownable::{cw_ownable, Ownership};

    #[cw_serde]
    pub struct InstantiateMsg {
        /// Address of the current contract owner
        pub owner: String,
    }

    #[cw_ownable]
    #[cw_serde]
    pub enum ExecuteMsg {}

    #[cw_serde]
    #[derive(QueryResponses)]
    pub enum QueryMsg {
        /// Return contract's ownership
        #[returns(Ownership<String>)]
        Ownership {},
    }
}
