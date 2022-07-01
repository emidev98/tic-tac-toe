#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps,Env, StdResult};

use crate::models::{QueryMsg, responses::GameResponse};
use crate::models::state::Game;
use crate::GAMES;
use cosmwasm_std::Order;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Games {
            host,
            opponent,
            completed
        } => to_binary(&query_games(deps, host, opponent, completed)?),
    }
}

fn query_games(deps: Deps, host: Option<String>, opponent: Option<String>, completed: Option<bool>) -> StdResult<GameResponse> {
    let mut games: Vec<Game>;
    let host_address = match &host {
        Some(host_address) => {
            match deps.api.addr_validate(&host_address) {
                Ok(addr) => Some(addr),
                Err(err) => return Err(err.into())
            }
        },
        None => None
    };
    let opponent_address = match &opponent {
        Some(opponent_address) => {
            match deps.api.addr_validate(&opponent_address) {
                Ok(addr) => Some(addr),
                Err(err) => return Err(err.into())
            }
        },
        None => None
    };

    if host_address.is_some() && opponent_address.is_some() {
        let match_option = GAMES
            .may_load(deps.storage,(&host_address.unwrap(), &opponent_address.unwrap()))
            .unwrap();

        match match_option {
            Some(_match) => games = vec![_match],
            None => games = vec![],   
        }
    } 
    else if host_address.is_some() {
        games = GAMES
            .prefix(&host_address.unwrap())
            .range(deps.storage, None, None, Order::Ascending)
            .map(|f| f.unwrap().1)
            .collect();
    } 
    else if opponent_address.is_some()  {
        games = GAMES
            .prefix(&opponent_address.unwrap())
            .range(deps.storage, None, None, Order::Ascending)
            .map(|f| f.unwrap().1)
            .collect();
    } 
    else {
        games = GAMES
            .range(deps.storage, None, None, Order::Ascending)
            .map(|f| f.unwrap().1)
            .collect();
    }

    if completed.is_some() {
        games = games
            .into_iter()
            .filter(|game| game.completed == completed.unwrap())
            .collect();
    }

    Ok(GameResponse {
        host,
        opponent,
        games,
    })
}
