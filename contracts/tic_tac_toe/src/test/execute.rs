use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Response, Addr};

use crate::contract::execute::execute;
use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::errors::ContractError;
use crate::models::{responses::GameResponse, ExecuteMsg, InstantiateMsg, QueryMsg, state::Game};

#[test]
fn create_game() {
    // GIVEN
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("host", &coins(2, "token"));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // WHEN
    let res_x = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::StartGame {
            x: 2,
            y: 0,
            host_symbol: true,
            opponent: String::from("opponent"),
        },
    );
    let execute_value: Response = res_x.unwrap();
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Games {
            host: Some(String::from("host")),
            opponent: Some(String::from("opponent")),
            completed: None,
        },
    );

    // THEN
    let query_value: GameResponse = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        query_value,
        GameResponse {
            host: Some(String::from("host")),
            opponent: Some(String::from("opponent")),
            games: vec![Game {
                board: vec![
                    vec![None, None, None],
                    vec![None, None, None],
                    vec![Some(true), None, None]
                ],
                host_symbol: true,
                prize: coins(2, "token"),
                completed: false
            }]
        }
    );
    assert_eq!(
        execute_value,
        Response::new()
            .add_attribute("method", "start_game")
            .add_attribute("x", "2")
            .add_attribute("y", "0")
            .add_attribute("host_symbol", "true")
            .add_attribute("opponent", "opponent")
    );
}

#[test]
fn fail_create_game_with_already_in_progress_game() {
    // GIVEN
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("host", &coins(2, "token"));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // WHEN
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::StartGame {
            x: 2,
            y: 0,
            host_symbol: true,
            opponent: String::from("opponent"),
        },
    ).unwrap();
    let res_x = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::StartGame {
            x: 2,
            y: 0,
            host_symbol: true,
            opponent: String::from("opponent"),
        },
    );
    let execute_value: ContractError = res_x.unwrap_err();

    // THEN
    assert_eq!(
        execute_value,
        ContractError::GameAlreadyInProgress {
            host: Addr::unchecked("host"),
            opponent: Addr::unchecked("opponent"),
        }
    );
}

#[test]
fn fail_create_games_with_wrong_coordinate() {
    // GIVEN
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("host", &coins(2, "token"));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // WHEN
    let res_x = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::StartGame {
            x: 3,
            y: 0,
            host_symbol: true,
            opponent: String::from("opponent"),
        },
    );

    let res_y = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::StartGame {
            x: 0,
            y: 3,
            host_symbol: true,
            opponent: String::from("opponent"),
        },
    );

    // THEN
    let value: ContractError = res_x.unwrap_err();
    assert_eq!(value, ContractError::InvalidCoordinates { x: 3, y: 0 });

    let value: ContractError = res_y.unwrap_err();
    assert_eq!(value, ContractError::InvalidCoordinates { x: 0, y: 3 });
}

#[test]
fn fail_create_games_against_itself() {
    // GIVEN
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("host", &coins(2, "token"));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // WHEN
    let res_x = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::StartGame {
            x: 0,
            y: 0,
            host_symbol: true,
            opponent: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = res_x.unwrap_err();
    assert_eq!(value, ContractError::CannotStartGame {});

}
