use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary};

use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{responses::GameResponse, InstantiateMsg, QueryMsg, QueryKey};

#[test]
fn empty_games_with_both_users() {
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
                opponent: String::from("opponent"),
            }),
            status: None,
        },
    );

    // THEN
    let value: Vec<GameResponse> = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        value,
        vec![]
    );
}

#[test]
fn empty_games_with_no_users() {
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
            key: None,
            status: None,
        },
    );

    // THEN
    let value: Vec<GameResponse> = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        value,
        vec![]
    );
}
