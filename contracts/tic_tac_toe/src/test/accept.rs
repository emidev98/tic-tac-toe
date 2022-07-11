use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Addr, Response, StdError};

use crate::contract::execute::execute;
use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{
    errors::ContractError, responses::GameResponse, state::Coord, state::Game, state::PlayerSymbol,
    state::Status, ExecuteMsg, InstantiateMsg, QueryMsg, QueryKey
};

#[test]
fn accept() {
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
        host_info,
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    )
    .unwrap();
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Games {
            key: Some(QueryKey {
                host: String::from("host"),
                opponent: String::from("opponent")
            }),
            status: Some(Status::PLAYING)
        },
    );

    // THEN
    let query_value: Vec<GameResponse> = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        opponent_response,
        Response::new()
            .add_attribute("method", "accept")
            .add_attribute("x", "1")
            .add_attribute("y", "1")
            .add_attribute("opponent", "host")
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
                    vec![None, None, None]
                ],
                player_round: Some(PlayerSymbol::X),
                host_symbol: PlayerSymbol::X,
                prize: coins(4, "token"),
                status: Status::PLAYING,
                winner: None
            }
        }]
    );
}

#[test]
fn accept_with_incorrect_host() {
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
        host_info,
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 2, y: 2 },
            host: String::from("w"),
        },
    );

    // THEN
    let value: ContractError = opponent_response.unwrap_err();
    assert_eq!(
        value,
        ContractError::Std(StdError::GenericErr {
            msg: String::from("Invalid input: human address too short")
        })
    );
}

#[test]
fn accept_inexistent() {
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

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 1, y: 1 },
            host: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = opponent_response.unwrap_err();
    assert_eq!(
        value,
        ContractError::InvalidGame {
            host: Addr::unchecked("opponent"),
            opponent: Addr::unchecked("host")
        }
    );
}

#[test]
fn accept_with_incorrect_coord() {
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
        host_info,
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 5, y: 5 },
            host: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = opponent_response.unwrap_err();
    assert_eq!(
        value,
        ContractError::InvalidCoord {
            coord: Coord { x: 5, y: 5 }
        }
    );
}

#[test]
fn accept_on_already_played_coords() {
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
        host_info,
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 2, y: 0 },
            host: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = opponent_response.unwrap_err();
    assert_eq!(
        value,
        ContractError::CoordinateAlreadyPlayed {
            coord: Coord { x: 2, y: 0 }
        }
    );
}

#[test]
fn accept_with_less_funds() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(1, "token"));
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
        host_info,
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 1, y: 2 },
            host: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = opponent_response.unwrap_err();
    assert_eq!(value, ContractError::InvalidReceivedFunds {});
}

#[test]
fn accept_with_more_funds() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(3, "token"));
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
        host_info,
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 1, y: 2 },
            host: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = opponent_response.unwrap_err();
    assert_eq!(value, ContractError::InvalidReceivedFunds {});
}

#[test]
fn accept_with_different_funds() {
    // GIVEN
    let mut deps = mock_dependencies();
    let host_info = mock_info("host", &coins(2, "token"));
    let opponent_info = mock_info("opponent", &coins(2, "w"));
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
        host_info,
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    // WHEN
    let opponent_response = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info,
        ExecuteMsg::Accept {
            coord: Coord { x: 1, y: 2 },
            host: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = opponent_response.unwrap_err();
    assert_eq!(value, ContractError::InvalidReceivedFunds {});
}
