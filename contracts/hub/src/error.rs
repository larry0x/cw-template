use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] cosmwasm_std::StdError),

    #[error("sender is not owner")]
    NotOwner,

    #[error("sender is not the pending owner")]
    NotPendingOwner,

    #[error("there is no current pending ownership transfer")]
    NoPendingOwnershipTransfer,
}
