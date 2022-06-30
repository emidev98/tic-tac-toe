#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::models::{
    ExecuteMsg,
    errors::ContractError
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Play {
            x,
            y,
            opponent
        } => try_play(deps, info, x, y, opponent),
    }
}

fn try_play(
    _deps: DepsMut, 
    _info: MessageInfo,
    x: u8,
    y: u8,
    opponent: Option<String>
) -> Result<Response, ContractError> {
    
    Ok(Response::new()
        .add_attribute("method", "reset"))
}
