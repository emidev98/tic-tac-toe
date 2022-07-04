use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Addr, Response, StdError};

use crate::contract::execute::execute;
use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{
    errors::ContractError, responses::GameResponse, state::Coord, state::Game, state::PlayerSymbol,
    state::Status, ExecuteMsg, InstantiateMsg, QueryMsg,
};

#[test]
fn play_round() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "token"));
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::CreateGame {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::AcceptGame {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    )
    .unwrap();

    // WHEN
    let play_res = execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 2, y: 2 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Games {
            host: Some(String::from("host")),
            opponent: Some(String::from("opponent")),
            status: Some(Status::PLAYING),
        },
    );

    // THEN
    let query_value: GameResponse = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        play_res,
        Response::new()
            .add_attribute("method", "play")
            .add_attribute("x", "2")
            .add_attribute("y", "2")
            .add_attribute("status", "PLAYING")
            .add_attribute("opponent", "opponent")
    );
    assert_eq!(
        query_value,
        GameResponse {
            host: Some(String::from("host")),
            opponent: Some(String::from("opponent")),
            games: vec![Game {
                board: vec![
                    vec![None, None, None],
                    vec![None, Some(PlayerSymbol::O), None],
                    vec![Some(PlayerSymbol::X), None, Some(PlayerSymbol::X)]
                ],
                player_round: PlayerSymbol::O,
                host_symbol: PlayerSymbol::X,
                prize: coins(4, "token"),
                status: Status::PLAYING,
                winner: None
            }]
        }
    );
}

#[test]
fn play_round_with_invalid_coords() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "token"));
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::CreateGame {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::AcceptGame {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    )
    .unwrap();

    // WHEN
    let play_res = execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 3, y: 3 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap_err();

    // THEN
    assert_eq!(
        play_res,
        ContractError::InvalidCoord {
            coord: Coord { x: 3, y: 3 }
        }
    );
}

#[test]
fn play_inexistent_round() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "token"));
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::CreateGame {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::AcceptGame {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    )
    .unwrap();

    // WHEN
    let play_res = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 1, y: 2 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap_err();

    // THEN
    assert_eq!(
        play_res,
        ContractError::InvalidGame {
            host: Addr::unchecked("opponent"),
            opponent: Addr::unchecked("opponent")
        }
    );
}

#[test]
fn play_invited_round() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "token"));
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::CreateGame {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let play_res = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 1, y: 2 },
            opponent: String::from("host"),
        },
    )
    .unwrap_err();

    // THEN
    assert_eq!(
        play_res,
        ContractError::InvalidGame {
            host: Addr::unchecked("opponent"),
            opponent: Addr::unchecked("host")
        }
    );
}

#[test]
fn play_round_on_existent_symbol() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "token"));
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::CreateGame {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::AcceptGame {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    )
    .unwrap();

    // WHEN
    let play_res = execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 1, y: 1 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap_err();

    // THEN
    assert_eq!(
        play_res,
        ContractError::CoordinateAlreadyPlayed {
            coord: Coord { x: 1, y: 1 }
        }
    );
}

#[test]
fn play_two_rounds_as_host() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "token"));
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::CreateGame {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::AcceptGame {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 2, y: 1 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let play_res = execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 0, y: 0 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap_err();

    // THEN
    assert_eq!(
        play_res,
        ContractError::TurnAlreadyPlayed {
            second_player: String::from("opponent")
        }
    );
}


#[test]
fn play_two_rounds_as_opponent() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "token"));
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::CreateGame {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::AcceptGame {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    )
    .unwrap();

    // WHEN
    let play_res = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Play {
            as_host: false,
            coord: Coord { x: 0, y: 0 },
            opponent: String::from("host"),
        },
    )
    .unwrap_err();

    // THEN
    assert_eq!(
        play_res,
        ContractError::TurnAlreadyPlayed {
            second_player: String::from("host")
        }
    );
}