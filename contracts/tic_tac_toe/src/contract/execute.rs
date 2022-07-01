#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{models::{errors::ContractError, state::Game, ExecuteMsg}, GAMES};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartGame {
            x,
            y,
            host_symbol,
            opponent,
        } => try_start_game(deps, info, x, y, host_symbol, opponent),
        ExecuteMsg::Play {
            x,
            y,
            opponent,
        } => try_play(deps, info, x, y, opponent),
    }
}

fn try_start_game(
    deps: DepsMut,
    info: MessageInfo,
    x: u8,
    y: u8,
    host_symbol: bool,
    opponent: String,
) -> Result<Response, ContractError> {
    let opponent_address = deps.api.addr_validate(&opponent)?;
    
    if opponent_address == info.sender {
        return Err(ContractError::CannotStartGame {});
    }

    let hosted_game_in_progress = GAMES
        .may_load(deps.storage,(&info.sender, &opponent_address))
        .unwrap()
        .filter(|game| !game.completed);

    let invited_game_in_progress = GAMES
        .may_load(deps.storage,(&opponent_address, &info.sender))
        .unwrap()
        .filter(|game| !game.completed);

    if hosted_game_in_progress.is_some() || invited_game_in_progress.is_some() {
        return Err(ContractError::GameAlreadyInProgress {
            host: info.sender,
            opponent: opponent_address,
        });
    }
    else {
        let game = Game::new(x, y, info.funds, host_symbol)?;
        GAMES.save(deps.storage, (&info.sender, &opponent_address), &game)?;
    }

    Ok(Response::new()
        .add_attribute("method", "start_game")
        .add_attribute("x", x.to_string())
        .add_attribute("y", y.to_string())
        .add_attribute("host_symbol", host_symbol.to_string())
        .add_attribute("opponent", opponent))
}

fn try_play(
    deps: DepsMut,
    info: MessageInfo,
    x: u8,
    y: u8,
    opponent: String,
) -> Result<Response, ContractError> {
    if x > 2 || y > 2 {
        return Err(ContractError::InvalidCoordinates { x, y });
    }
    let opponent_address = deps.api.addr_validate(&opponent)?;

    let hosted_game_in_progress = GAMES
        .may_load(deps.storage,(&info.sender, &opponent_address))
        .unwrap()
        .filter(|game| !game.completed);

    let invited_game_in_progress = GAMES
        .may_load(deps.storage,(&opponent_address, &info.sender))
        .unwrap()
        .filter(|game| !game.completed);

    if invited_game_in_progress.is_some() {

    }
    else if hosted_game_in_progress.is_some() {

    }
    else {
        return Err(ContractError::GameAlreadyCompleted {
            host: info.sender,
            opponent: opponent_address,
        });
    }
    
    Ok(Response::new()
        .add_attribute("method", "play")
        .add_attribute("x", x.to_string())
        .add_attribute("y", y.to_string())
        .add_attribute("opponent", opponent))
}
