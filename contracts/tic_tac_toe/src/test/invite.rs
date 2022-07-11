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
fn invite() {
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
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    );
    let execute_value: Response = res_x.unwrap();
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Games {
            key: Some(QueryKey {
                host: String::from("host"),
                opponent: String::from("opponent")
            }),
            status: Some(Status::INVITED),
        },
    );

    // THEN
    let query_value: Vec<GameResponse> = from_binary(&res.unwrap()).unwrap();
    assert_eq!(
        query_value,
        vec![GameResponse {
            host: Addr::unchecked("host"),
            opponent: Addr::unchecked("opponent"),
            game: Game {
                board: vec![
                    vec![None, None, Some(PlayerSymbol::X)],
                    vec![None, None, None],
                    vec![None, None, None]
                ],
                player_round: Some(PlayerSymbol::O),
                host_symbol: PlayerSymbol::X,
                prize: coins(2, "token"),
                status: Status::INVITED,
                winner: None
            }
        }]
    );
    assert_eq!(
        execute_value,
        Response::new()
            .add_attribute("method", "invite")
            .add_attribute("x", "2")
            .add_attribute("y", "0")
            .add_attribute("host_symbol", "X")
            .add_attribute("opponent", "opponent")
    );
}
#[test]
fn invite_when_already_in_progress_game() {
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
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    )
    .unwrap();
    let res_x = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::Invite {
            coord: Coord { x: 2, y: 2 },
            host_symbol: PlayerSymbol::X,
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
fn invite_when_wrong_coordinate() {
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
        ExecuteMsg::Invite {
            coord: Coord { x: 3, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    );

    let res_y = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("host", &coins(2, "token")),
        ExecuteMsg::Invite {
            coord: Coord { x: 0, y: 3 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("opponent"),
        },
    );

    // THEN
    let value: ContractError = res_x.unwrap_err();
    assert_eq!(
        value,
        ContractError::InvalidCoord {
            coord: Coord { x: 3, y: 0 }
        }
    );

    let value: ContractError = res_y.unwrap_err();
    assert_eq!(
        value,
        ContractError::InvalidCoord {
            coord: Coord { x: 0, y: 3 }
        }
    );
}

#[test]
fn invite_against_itself() {
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
        ExecuteMsg::Invite {
            coord: Coord { x: 0, y: 0 },
            host_symbol: PlayerSymbol::X,
            opponent: String::from("host"),
        },
    );

    // THEN
    let value: ContractError = res_x.unwrap_err();
    assert_eq!(value, ContractError::CannotStartGame {});
}
