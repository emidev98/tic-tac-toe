use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary};

use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{responses::GameResponse, InstantiateMsg, QueryMsg};

#[test]
fn empty_games_by_host() {
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
            host: Some(String::from("host")),
            opponent: None,
            status: None,
        },
    );

    // THEN
    let value: GameResponse = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        value,
        GameResponse {
            host: Some(String::from("host")),
            opponent: None,
            games: vec![]
        }
    );
}

#[test]
fn empty_games_by_opponent() {
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
            host: None,
            opponent: Some(String::from("opponent")),
            status: None,
        },
    );

    // THEN
    let value: GameResponse = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        value,
        GameResponse {
            host: None,
            opponent: Some(String::from("opponent")),
            games: vec![]
        }
    );
}

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
            host: Some(String::from("host")),
            opponent: Some(String::from("opponent")),
            status: None,
        },
    );

    // THEN
    let value: GameResponse = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        value,
        GameResponse {
            host: Some(String::from("host")),
            opponent: Some(String::from("opponent")),
            games: vec![]
        }
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
            host: None,
            opponent: None,
            status: None,
        },
    );

    // THEN
    let value: GameResponse = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        value,
        GameResponse {
            host: None,
            opponent: None,
            games: vec![]
        }
    );
}