use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Addr, Response, SubMsg, CosmosMsg, BankMsg};

use crate::contract::execute::execute;
use crate::contract::instantiate::instantiate;
use crate::contract::query::query;
use crate::models::{
    errors::ContractError, responses::GameResponse, state::Coord, state::Game, state::PlayerSymbol,
    state::Status, ExecuteMsg, InstantiateMsg, QueryMsg,
};

#[test]
fn reject_round() {
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
    let reject_res = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Reject {
            as_host: false,
            opponent: String::from("host"),
        },
    )
    .unwrap();

    // THEN
    assert_eq!(
        reject_res,
        Response::new()
            .add_attribute("method", "reject")
            .add_attribute("opponent", "host")
            .add_submessage(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: String::from("host"),
                amount: coins(2, "token"),
            })))
    );
}


#[test]
fn reject_inexistent_existent_round() {
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
    let reject_res = execute(
        deps.as_mut(),
        mock_env(),
        opponent_info.clone(),
        ExecuteMsg::Reject {
            as_host: false,
            opponent: String::from("host"),
        },
    )
    .unwrap_err();

    // THEN
    assert_eq!(reject_res, ContractError::GameNotFound {
        host: Addr::unchecked("opponent"),
        opponent: Addr::unchecked("host"),
    });
    
}

