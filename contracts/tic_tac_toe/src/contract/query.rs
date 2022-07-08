#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

use crate::models::state::Status;
use crate::models::QueryKey;
use crate::models::{responses::GameResponse, QueryMsg};
use crate::GAMES;
use cosmwasm_std::Order;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Games { key, status } => to_binary(&query_games(deps, key, status)?),
    }
}

fn query_games(
    deps: Deps,
    key: Option<QueryKey>,
    status: Option<Status>,
) -> StdResult<Vec<GameResponse>> {
    let mut res: Vec<GameResponse>;
    
    match key {
        Some(addresses) => {
            let host_address = deps.api.addr_validate(&addresses.host)?;
            let opponent_address = deps.api.addr_validate(&addresses.opponent)?;

            let game_option = GAMES
                .may_load(deps.storage, (&host_address, &opponent_address))
                .unwrap();

            match game_option {
                Some(_game) => {
                    res = vec![GameResponse {
                        game: _game,
                        host: host_address,
                        opponent: opponent_address,
                    }]
                }
                None => res = vec![],
            }
        }
        None => {
            res = GAMES
                .range(deps.storage, None, None, Order::Ascending)
                .map(|f| {
                    let (addresses, game) = f.unwrap();

                    GameResponse {
                        game: game,
                        host: addresses.0,
                        opponent: addresses.1,
                    }
                })
                .collect();
        }
    }

    match status {
        Some(status) => {
            res = res
                .into_iter()
                .filter(|res| res.game.status == status)
                .collect()
        }
        None => {}
    }

    Ok(res)
}
