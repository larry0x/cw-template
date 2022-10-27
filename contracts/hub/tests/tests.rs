use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Empty, OwnedDeps,
};

use steak::hub::{Config, InstantiateMsg};
use steak_hub::{contract, execute, query, error::ContractError};

fn setup_test() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = mock_dependencies();

    contract::instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("larry", &[]),
        InstantiateMsg {
            owner: "larry".into(),
        },
    )
    .unwrap();

    deps
}

#[test]
fn initializing() {
    let deps = setup_test();

    let cfg = query::config(deps.as_ref()).unwrap();
    assert_eq!(
        cfg,
        Config {
            owner: "larry".into(),
            pending_owner: None,
        },
    );
}

#[test]
fn setting_owner() {
    let mut deps = setup_test();

    execute::set_owner(deps.as_mut(), "jake".into()).unwrap();

    let cfg = query::config(deps.as_ref()).unwrap();
    assert_eq!(
        cfg,
        Config {
            owner: "jake".into(),
            pending_owner: None,
        },
    );
}

#[test]
fn transferring_ownership() {
    let mut deps = setup_test();

    // only owner can propose ownership transferrs
    {
        let err = execute::transfer_ownership(
            deps.as_mut(),
            mock_info("pumpkin", &[]),
            "pumpkin".into(),
        )
        .unwrap_err();
        assert_eq!(err, ContractError::NotOwner);
    }

    // tne owner properly proposes an ownership transfer
    {
        execute::transfer_ownership(
            deps.as_mut(),
            mock_info("larry", &[]),
            "jake".into(),
        )
        .unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(cfg.pending_owner, Some("jake".into()));
    }
}

#[test]
fn accepting_ownership() {
    let mut deps = setup_test();

    // attempt to accept ownership when there isn't a pending ownership transfer yet
    {
        let err = execute::accept_ownership(deps.as_mut(), mock_info("pumpkin", &[])).unwrap_err();
        assert_eq!(err, ContractError::NoPendingOwnershipTransfer);
    }

    execute::transfer_ownership(
        deps.as_mut(),
        mock_info("larry", &[]),
        "jake".into(),
    )
    .unwrap();

    // only the pending owner can accept ownership
    {
        let err = execute::accept_ownership(deps.as_mut(), mock_info("pumpkin", &[])).unwrap_err();
        assert_eq!(err, ContractError::NotPendingOwner);
    }

    // the pending owner properly accepts ownership
    {
        execute::accept_ownership(deps.as_mut(), mock_info("jake", &[])).unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(
            cfg,
            Config {
                owner: "jake".into(),
                pending_owner: None,
            },
        );
    }
}
