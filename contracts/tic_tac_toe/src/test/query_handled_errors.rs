use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, StdError};

use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{InstantiateMsg, QueryMsg, QueryKey};

#[test]
fn query_by_invalid_host() {
    // GIVEN
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("host", &coins(2, "token"));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // WHEN
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Games {
            key: Some(QueryKey {
                host: String::from("w"),
                opponent: String::from("opponent"),
            }),
            status: None,
        },
    );

    // THEN
    assert_eq!(
        res.unwrap_err(),
        StdError::GenericErr {
            msg: String::from("Invalid input: human address too short")
        }
    );
}

#[test]
fn query_by_invalid_opponent() {
    // GIVEN
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("host", &coins(2, "token"));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // WHEN
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Games {
            key: Some(QueryKey {
                host: String::from("host"),
                opponent: String::from("w"),
            }),
            status: None,
        },
    );

    // THEN
    assert_eq!(
        res.unwrap_err(),
        StdError::GenericErr {
            msg: String::from("Invalid input: human address too short")
        }
    );
}
