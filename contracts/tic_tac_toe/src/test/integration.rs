use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, BankMsg, Response, Addr};

use crate::contract::execute::execute;
use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{
    responses::GameResponse, state::Coord, state::Game, state::PlayerSymbol,
    state::Status, ExecuteMsg, InstantiateMsg, QueryMsg, QueryKey
};

#[test]
fn tie() {
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
            coord: Coord { x: 1, y: 1 },
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
            coord: Coord { x: 2, y: 2 },
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
            coord: Coord { x: 2, y: 0 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Play {
            as_host: false,
            coord: Coord { x: 0, y: 2 },
            opponent: String::from("host"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 1, y: 2 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Play {
            as_host: false,
            coord: Coord { x: 1, y: 0 },
            opponent: String::from("host"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        ExecuteMsg::Play {
            as_host: true,
            coord: Coord { x: 0, y: 0 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Play {
            as_host: false,
            coord: Coord { x: 2, y: 1 },
            opponent: String::from("host"),
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
            coord: Coord { x: 0, y: 1 },
            opponent: String::from("opponent"),
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
            status: Some(Status::COMPLETED),
        },
    );

    // THEN
    let query_value: Vec<GameResponse> = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        play_res,
        Response::new()
            .add_attribute("method", "play")
            .add_attribute("x", "0")
            .add_attribute("y", "1")
            .add_attribute("status", "COMPLETED")
            .add_attribute("opponent", "opponent")
            .add_messages(vec![
                BankMsg::Send {
                    to_address: String::from("host"),
                    amount: coins(2, "token"),
                },
                BankMsg::Send {
                    to_address: String::from("opponent"),
                    amount: coins(2, "token"),
                },
            ])
    );
    assert_eq!(
        query_value,
        vec![GameResponse {
            host: Addr::unchecked("host"),
            opponent: Addr::unchecked("opponent"),
            game: Game {
                board: vec![
                    vec![Some(PlayerSymbol::X), Some(PlayerSymbol::O), Some(PlayerSymbol::X)],
                    vec![Some(PlayerSymbol::X), Some(PlayerSymbol::X), Some(PlayerSymbol::O)],
                    vec![Some(PlayerSymbol::O), Some(PlayerSymbol::X), Some(PlayerSymbol::O)]
                ],
                player_round: PlayerSymbol::X,
                host_symbol: PlayerSymbol::X,
                prize: coins(4, "token"),
                status: Status::COMPLETED,
                winner: None
            }
        }]
    );
}

#[test]
fn host_wins() {
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
            coord: Coord { x: 1, y: 1 },
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
            coord: Coord { x: 2, y: 2 },
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
            coord: Coord { x: 1, y: 0 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Play {
            as_host: false,
            coord: Coord { x: 0, y: 2 },
            opponent: String::from("host"),
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
            coord: Coord { x: 1, y: 2 },
            opponent: String::from("opponent"),
        },
    )
    .unwrap();

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Games {
            key : Some(QueryKey {
                host: String::from("host"),
                opponent: String::from("opponent")
            }),
            status: Some(Status::COMPLETED),
        },
    );

    // THEN
    let query_value: Vec<GameResponse> = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        play_res,
        Response::new()
            .add_attribute("method", "play")
            .add_attribute("x", "1")
            .add_attribute("y", "2")
            .add_attribute("status", "COMPLETED")
            .add_attribute("opponent", "opponent")
            .add_attribute("winner", "X")
            .add_messages(vec![
                BankMsg::Send {
                    to_address: String::from("host"),
                    amount: coins(4, "token"),
                },
            ])
    );
    assert_eq!(
        query_value,
        vec![GameResponse {
            host: Addr::unchecked("host"),
            opponent: Addr::unchecked("opponent"),
            game:Game {
                board: vec![
                    vec![None, Some(PlayerSymbol::X), None], 
                    vec![None, Some(PlayerSymbol::X), None], 
                    vec![Some(PlayerSymbol::O), Some(PlayerSymbol::X), Some(PlayerSymbol::O)]
                ],
                player_round: PlayerSymbol::X,
                host_symbol: PlayerSymbol::X,
                prize: coins(4, "token"),
                status: Status::COMPLETED,
                winner: Some(PlayerSymbol::X)
            }
        }]
    );
}
