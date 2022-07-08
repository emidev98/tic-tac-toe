use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Addr, Response};

use crate::contract::execute::execute;
use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{
    errors::ContractError, responses::GameResponse, state::Coord, state::Game, state::PlayerSymbol,
    state::Status, ExecuteMsg, InstantiateMsg, QueryMsg, QueryKey
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
        ExecuteMsg::Invite {
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
        ExecuteMsg::Accept {
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
            key : Some( QueryKey {
                host: String::from("host"),
                opponent: String::from("opponent"),
            }),
            status: Some(Status::PLAYING),
        },
    );

    // THEN
    let query_value: Vec<GameResponse> = from_binary(&res.unwrap()).unwrap();
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
        vec![GameResponse {
            host: Addr::unchecked("host"),
            opponent: Addr::unchecked("opponent"),
            game: Game {
                board: vec![
                    vec![None, None, Some(PlayerSymbol::X)],
                    vec![None, Some(PlayerSymbol::O), None],
                    vec![None, None, Some(PlayerSymbol::X)]
                ],
                player_round: PlayerSymbol::O,
                host_symbol: PlayerSymbol::X,
                prize: coins(4, "token"),
                status: Status::PLAYING,
                winner: None
            }
        }]
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
        ExecuteMsg::Invite {
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
        ExecuteMsg::Accept {
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
        ExecuteMsg::Invite {
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
        ExecuteMsg::Accept {
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
        ExecuteMsg::Invite {
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
        ExecuteMsg::Invite {
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
        ExecuteMsg::Accept {
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
        ExecuteMsg::Invite {
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
        ExecuteMsg::Accept {
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
        ExecuteMsg::Invite {
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
        ExecuteMsg::Accept {
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
