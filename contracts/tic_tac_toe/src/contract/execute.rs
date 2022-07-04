#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg};

use crate::{
    models::{
        errors::ContractError,
        state::{Coord, Game, PlayerSymbol, Status},
        ExecuteMsg,
    },
    GAMES,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateGame {
            coord,
            host_symbol,
            opponent,
        } => try_create_game(deps, info, coord, host_symbol, opponent),
        ExecuteMsg::AcceptGame { coord, host } => try_accept_game(deps, info, coord, host),
        ExecuteMsg::Play {
            as_host,
            coord,
            opponent,
        } => try_play(deps, info, as_host, coord, opponent),
    }
}

fn try_create_game(
    deps: DepsMut,
    info: MessageInfo,
    coord: Coord,
    host_symbol: PlayerSymbol,
    opponent: String,
) -> Result<Response, ContractError> {
    let opponent_address = deps.api.addr_validate(&opponent)?;
    if !coord.is_valid() {
        return Err(ContractError::InvalidCoord { coord });
    }

    if opponent_address == info.sender {
        return Err(ContractError::CannotStartGame {});
    }

    let in_progress_hosted_game = GAMES
        .may_load(deps.storage, (&info.sender, &opponent_address))
        .unwrap()
        .filter(|game| game.status == Status::PLAYING || game.status == Status::INVITED);

    let in_progress_invited_game = GAMES
        .may_load(deps.storage, (&opponent_address, &info.sender))
        .unwrap()
        .filter(|game| game.status == Status::PLAYING || game.status == Status::INVITED);

    if in_progress_hosted_game.is_some() || in_progress_invited_game.is_some() {
        return Err(ContractError::GameAlreadyInProgress {
            host: info.sender,
            opponent: opponent_address,
        });
    } else {
        let game = Game::new(coord, info.funds, host_symbol.clone());
        GAMES.save(deps.storage, (&info.sender, &opponent_address), &game)?;
    }

    Ok(Response::new()
        .add_attribute("method", "create_game")
        .add_attribute("x", coord.x.to_string())
        .add_attribute("y", coord.y.to_string())
        .add_attribute("host_symbol", host_symbol.to_string())
        .add_attribute("opponent", opponent))
}

fn try_accept_game(
    deps: DepsMut,
    info: MessageInfo,
    coord: Coord,
    host: String,
) -> Result<Response, ContractError> {
    let host_address = deps.api.addr_validate(&host)?;
    if !coord.is_valid() {
        return Err(ContractError::InvalidCoord { coord });
    }

    let game = GAMES
        .may_load(deps.storage, (&host_address, &info.sender))
        .unwrap()
        .filter(|game| game.status == Status::INVITED);

    if game.is_none() {
        return Err(ContractError::InvalidGame {
            host: info.sender,
            opponent: host_address,
        });
    } else {
        let mut game = game.unwrap();
        if game.already_played_on(coord) {
            return Err(ContractError::CoordinateAlreadyPlayed { coord });
        } else if game.prize.ne(&info.funds) {
            return Err(ContractError::InvalidReceivedFunds {});
        }
        let game = game.double_prize().play(coord).finish_round();
        game.status = Status::PLAYING;

        GAMES.save(deps.storage, (&host_address, &info.sender), game)?;
    }

    Ok(Response::new()
        .add_attribute("method", "accept_game")
        .add_attribute("x", coord.x.to_string())
        .add_attribute("y", coord.y.to_string())
        .add_attribute("opponent", host_address))
}

fn try_play(
    deps: DepsMut,
    info: MessageInfo,
    as_host: bool,
    coord: Coord,
    opponent: String,
) -> Result<Response, ContractError> {
    let opponent_address = deps.api.addr_validate(&opponent)?;
    if !coord.is_valid() {
        return Err(ContractError::InvalidCoord { coord });
    }
    let key = if as_host {
        (&info.sender, &opponent_address)
    } else {
        (&opponent_address, &info.sender)
    };

    let game = GAMES
        .may_load(deps.storage, key)
        .unwrap()
        .filter(|game| game.status == Status::PLAYING);

    if game.is_none() {
        return Err(ContractError::InvalidGame {
            host: info.sender,
            opponent: opponent_address,
        });
    } else {
        let mut game = game.unwrap();
        if game.already_played_on(coord) {
            return Err(ContractError::CoordinateAlreadyPlayed { coord });
        } else if game.already_played(as_host) {
            return Err(ContractError::TurnAlreadyPlayed { second_player: opponent });
        }

        let game = game.play(coord);

        if game.is_current_player_winner() {
            game.status = Status::COMPLETED;
            game.winner = Some(game.player_round);
        } else if game.is_full_board() {
            game.status = Status::COMPLETED;
        } else {
            game.finish_round();
        }

        GAMES.save(deps.storage, key, game)?;

        let res = Response::new()
            .add_attribute("method", "play")
            .add_attribute("x", coord.x.to_string())
            .add_attribute("y", coord.y.to_string())
            .add_attribute("status", game.status.to_string())
            .add_attribute("opponent", opponent);

        if game.status == Status::COMPLETED {
            if game.winner.is_some() {
                let winner_bank_message = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: game.prize.clone(),
                }));

                return Ok(res
                    .add_attribute("winner", game.winner.unwrap().to_string())
                    .add_submessage(winner_bank_message));
            } else {
                let prize = game.get_half_prize();
                let host_bank_message = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: prize.clone(),
                }));
                let opponent_bank_message = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: prize.clone(),
                }));

                return Ok(res
                    .add_attribute("winner", game.winner.unwrap().to_string())
                    .add_submessages(vec![host_bank_message, opponent_bank_message]));
            }
        }

        Ok(res)
    }
}
