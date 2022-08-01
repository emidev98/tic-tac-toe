# Tic Tac Toe

The decision to build this game is because does not require a low latency to play and it is a good challenge that covers most of the concepts of the blockchain.

## Modules Structure

The smart contract is divided in three main modules:

- **contract** module contains the contract logic,
- **models** module contains the data structures and models of the contract,
- **test** contains the testing logic for the contract.

Each module has its own sub modules that are used to divide the logic per operational domains, e.g. state modification are defined in **src/contract/execute.rs**, responses models are defined in **src/models/responses.rs** module...

```
.
├── artifacts
├── examples
│   └── schema.rs
└── src
    ├── lib.rs
    ├── models
    │   ├── errors.rs
    │   ├── mod.rs
    │   ├── responses.rs
    │   └── state.rs
    ├── contract
    │   ├── execute.rs
    │   ├── instantiate.rs
    │   ├── mod.rs
    │   └── query.rs
    └── test
        ├── accept.rs
        ├── happy_paths.rs
        ├── invite.rs
        ├── mod.rs
        ├── play.rs
        ├── query_handled_errors.rs
        ├── query_happy_path.rs
        └── reject.rs
```

## Models

The main model of this contract is the Game located in **state.rs** module because it contains the properties of a match, the logic of playing the game and its stored into the contract state. Aside of the game model you can find the Coord model composed by x and y properties containing the coordinates of a cell in the game. PlayerSymbol which is used to be a enum with the symbols of the players. Finally you can also find the Status enum used to define the different game states:

- **INVITED** only one game can be in this state at a time per host and opponent pair. This state is achieved by creating a new game and the following possible state are PLAYING or REJECTED.
- **PLAYING** only one game can be in this state at a time per host and opponent pair. To achieve this state must mutate from INVITED.
- **COMPLETED** multiple games can be in this state but they have to mutate from PLAYING.
- **REJECTED** multiple games can be in this state but they have to mutate from INVITE. 

As you may already have noticed this game can only be played by 1 host and 1 opponent at a time (host being the one who created the game) but when a match is completed or rejected new game can be started.

> src/models/state.rs
```rs
use std::fmt;

use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
    /**
        Board is defined as following due the
        smart contract optimization process:
        - None: can be selected,
        - Some(Symbol.O): already selected by 'O',
        - Some(Symbol.X): already selected by 'X'

        None|None|None
        ----|----|----
        None|None|None
        ----|----|----
        None|None|None
    */
    pub board: Vec<Vec<Option<PlayerSymbol>>>,

    /**
        Used to determine the symbol of the player that
        started the game an the symbol of the opponent.
    */
    pub host_symbol: PlayerSymbol,

    /*
        Semaphore to determine who's current and next turn.
    */
    pub player_round: Option<PlayerSymbol>,

    /**
       Tracks the amount of coins that will have
       to be transfer to the winner of the game.
    */
    pub prize: Vec<Coin>,

    /**
        Determine the game status, where
        - INVITED: only one game can be in this status at a time per host and opponent pair. This status is achieved by creating a new game and the following possible status are PLAYING or REJECTED.
        - PLAYING: only one game can be in this status at a time per host and opponent pair. To achieve this status must mutate from INVITED.
        - COMPLETED: multiple games can be in this status but they have to mutate from PLAYING.
        - REJECTED: multiple games can be in this status but they have to mutate from INVITE. 
    */
    pub status: Status,

    /**
        Keeps track of the game winner based on the property 
        status from Game struct. When status is COMPLETED,
        winner means the following:
        - None: means that is tie
        - Some(PlayerSymbol.X): player X won,
        - Some(PlayerSymbol.O): player O won
    */
    pub winner: Option<PlayerSymbol>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub enum PlayerSymbol {
    X,
    O
}

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerSymbol::X => write!(f, "X"),
            PlayerSymbol::O => write!(f, "O"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub enum Status {
    INVITED,
    PLAYING,
    COMPLETED,
    REJECTED,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::INVITED => write!(f, "INVITED"),
            Status::PLAYING => write!(f, "PLAYING"),
            Status::COMPLETED => write!(f, "COMPLETED"),
            Status::REJECTED => write!(f, "REJECTED"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    pub fn is_valid(self) -> bool {
        self.x < 3 && self.y < 3
    }
}

impl Game {

    pub fn new(coord: Coord, prize: Vec<Coin>, host_symbol: PlayerSymbol) -> Game {
        let mut symbol_round = PlayerSymbol::O;
        let mut board = vec![vec![None; 3]; 3];
        let row = board.get_mut(coord.y as usize).unwrap();
        row[coord.x as usize] = Some(host_symbol);

        if host_symbol == PlayerSymbol::O {
            symbol_round = PlayerSymbol::X;
        }

        Game {
            board,
            host_symbol,
            player_round: Some(symbol_round),
            prize,
            status: Status::INVITED,
            winner: None,
        }
    }

    pub fn already_played_on(&self, coord: Coord) -> bool {
        return self
            .board
            .get(coord.y as usize)
            .unwrap()
            .get(coord.x as usize)
            .unwrap()
            .is_some();
    }

    pub fn already_played(&mut self, as_host: bool) -> bool {
        match self.player_round {
            Some(current_player_symbol) => {
                if as_host { 
                    return current_player_symbol != self.host_symbol 
                } else {
                    return current_player_symbol == self.host_symbol
                }

            }
            None => return false,
        }
    }

    pub fn double_prize(&mut self) -> &mut Game {
        for coin in &mut self.prize {
            coin.amount = coin.amount.checked_mul(Uint128::new(2)).unwrap();
        }

        self
    }

    pub fn get_half_prize(&self) -> Vec<Coin> {
        self.prize.clone()
            .iter_mut()
            .map(|coin| {
                coin.amount = coin.amount.checked_div(Uint128::new(2)).unwrap();
                coin.clone()
            })
            .collect()
        
    }

    pub fn play(&mut self, coord: Coord) -> &mut Game {
        let row = self.board.get_mut(coord.y as usize).unwrap();
        row[coord.x as usize] = self.player_round;

        self
    }

    pub fn finish_round(&mut self) -> &mut Game {
        match self.player_round {
            Some(PlayerSymbol::X) => self.player_round = Some(PlayerSymbol::O),
            Some(PlayerSymbol::O) => self.player_round = Some(PlayerSymbol::X),
            None => panic!("Invalid player round"),
        };

        self
    }

    pub fn is_full_board(&self) -> bool {
        for row in &self.board {
            for cell in row {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_current_player_winner(&self) -> bool {
        macro_rules! has {
            ($y:expr, $x:expr) => {
                self.board[$y][$x] == self.player_round
            };
        }
        
        // Horizontally
        for row in 0..3 {
            if has!(row, 0) && has!(row, 1) && has!(row, 2) {
                return true;
            }
        }

        // Vertically
        for col in 0..3 {
            if has!(0, col) && has!(1, col) && has!(2, col) {
                return true;
            }
        }

        // Top-left to bottom-right
        if has!(0, 0) && has!(1, 1) && has!(2, 2) {
            return true;
        }

        // Top-right to bottom-left
        if has!(0, 2) && has!(1, 1) && has!(2, 0) {
            return true;
        }
        
        false
    }
}
```

The following module contains the data structure for quey responses:

> src/models/responses.rs
```rs
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Game;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameResponse {
    pub game: Game,
    pub host: Addr,
    pub opponent: Addr
}
```

Contains the data structures for error cases with its own description:
> src/models/errors.rs
```rs
use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

use super::state::Coord;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Game against yourself cannot be started")]
    CannotStartGame {},

    #[error("Game between {host} and {opponent} already exists. Complete the previous game to start a new one")]
    GameAlreadyInProgress { host: Addr, opponent: Addr },

    #[error("Game between {host} and {opponent} not found. You cannot reject it")]
    GameNotFound { host: Addr, opponent: Addr },

    #[error("Game between {host} and {opponent} is invalid. Try starting another game.")]
    InvalidGame { host: Addr, opponent: Addr },

    #[error("x={{coord.x}} and y={{coord.y}} already contain symbol. Try using another coordinate.")]
    CoordinateAlreadyPlayed { coord: Coord },

    #[error("You already played this turn. Wait for '{{second_player}}' to play its turn.")]
    TurnAlreadyPlayed { second_player: String},

    #[error("Invalid coordinate x={{coord.x}} y={{coord.y}}. Coordinates must be between 0 and 2")]
    InvalidCoord { coord: Coord },

    #[error("The funds you send must be equal to the prize of the game")]
    InvalidReceivedFunds {},
}
```

The last part of this module is the entry point which also contains the contract entry points like InstantiateMsg, ExecuteMsg, QueryMsg or QueryKey which is a direct dependency on QueryMsg:

> src/models/mod.rs
```rs
pub mod errors;
pub mod responses;
pub mod state;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use self::state::{PlayerSymbol, Status, Coord};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Invite {
        coord: Coord,
        host_symbol: PlayerSymbol,
        opponent: String
    },
    Reject {
        as_host: bool,
        opponent: String
    },
    Accept {
        coord: Coord,
        host: String
    },
    Play {
        as_host: bool,
        coord: Coord,
        opponent: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Games {
        key: Option<QueryKey>,
        status: Option<Status>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryKey {
    pub host: String,
    pub opponent: String,
}
```

## Contract

This module is divided in three sub modules,
- **mod** is the rust main entry point that exports the three following modules,
- **instantiate** module in charge to giving an initial state to the contract,
- **execute** module in charge to modifying the state of the contract when a user submits a transaction, 
- **query** module in charge to read the state of the contract,

> src/contract/mod.rs
```rs
pub mod instantiate;
pub mod execute;
pub mod query;
```

This module is almost empty but as every contract must contain an instantiate method that cannot be implicit here you can find the following code:

> src/contract/instantiate.rs
```rs
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::models::{
    InstantiateMsg,
    errors::ContractError
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}
```

There are only three ways to interact with this contract are by executing:

- **Invite** will create a game in INVITED status between two players as the player that have submitted the transaction being the host and the opponent being the address that is competing against. The use that send the transaction have to select its symbol and also the initial coordinates.

- **Reject** can only be executed when a game is in status INVITED. This action will return the funds to the player that has created the Invitation and will set the status to REJECTED. This method can be executed by both players as a way to reject that game by sending as_host parameter to true if you are rejecting the match you have invited an opponent to.

- **Accept** can only be executed when status INVITED. This action request to match the funds send by the host player that has created the game invitation. It received the coords and opponent address to play against (which must be the host address of the match). 

- **Play** method executed by both host and opponent to play the game. The parameter **as_host** should be send to true if you are playing from the host perspective or false if you are playing from the opponent. The parameter **coord** must be the coordinate where you want to play in the game field. Finally the last parameter **opponent** must also follow the same approach the parameter "as_host" does by sending the address of the opponent player from the current player perspective. This method is also in charge of tracking the status of the match, the winner of the match and when a match is finish is also in charge of returning the funds to the player that won or as in case of tie, to both players.

> src/contract/execute.rs
```rs
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, BankMsg, DepsMut, Env, MessageInfo, Response};

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
        ExecuteMsg::Invite {
            coord,
            host_symbol,
            opponent,
        } => try_invite(deps, info, coord, host_symbol, opponent),
        ExecuteMsg::Reject { as_host, opponent } => try_reject(deps, info, as_host, opponent),
        ExecuteMsg::Accept { coord, host } => try_accept(deps, info, coord, host),
        ExecuteMsg::Play {
            as_host,
            coord,
            opponent,
        } => try_play(deps, info, as_host, coord, opponent),
    }
}

fn try_invite(
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
        .add_attribute("method", "invite")
        .add_attribute("x", coord.x.to_string())
        .add_attribute("y", coord.y.to_string())
        .add_attribute("host_symbol", host_symbol.to_string())
        .add_attribute("opponent", opponent))
}

fn try_reject(
    deps: DepsMut,
    info: MessageInfo,
    as_host: bool,
    opponent: String,
) -> Result<Response, ContractError> {
    let opponent_address = deps.api.addr_validate(&opponent)?;
    let key: (&Addr, &Addr);
    let refund_address: &Addr;

    if as_host {
        key = (&info.sender, &opponent_address);
        refund_address = &info.sender;
    } else {
        key = (&opponent_address, &info.sender);
        refund_address = &opponent_address;
    };

    let game = GAMES
        .may_load(deps.storage, key)
        .unwrap()
        .filter(|game| game.status == Status::INVITED);

    if game.is_none() {
        return Err(ContractError::GameNotFound {
            host: info.sender,
            opponent: opponent_address,
        });
    } else {
        let mut game = game.unwrap();
        game.status = Status::REJECTED;
        GAMES.save(deps.storage, key, &game)?;

        Ok(Response::new()
            .add_attribute("method", "reject")
            .add_attribute("opponent", opponent)
            .add_message(BankMsg::Send {
                to_address: refund_address.to_string(),
                amount: game.prize.clone(),
            }))
    }
}

fn try_accept(
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
        .add_attribute("method", "accept")
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
            return Err(ContractError::TurnAlreadyPlayed {
                second_player: opponent,
            });
        }

        let game = game.play(coord);

        if game.is_current_player_winner() {
            game.status = Status::COMPLETED;
            game.winner = Some(game.player_round.unwrap());
            game.player_round = None;
        } else if game.is_full_board() {
            game.status = Status::COMPLETED;
            game.player_round = None;
        } else {
            game.finish_round();
        }

        GAMES.save(deps.storage, key, game)?;

        let res = Response::new()
            .add_attribute("method", "play")
            .add_attribute("x", coord.x.to_string())
            .add_attribute("y", coord.y.to_string())
            .add_attribute("status", game.status.to_string())
            .add_attribute("opponent", opponent.clone());

        if game.status == Status::COMPLETED {
            if game.winner.is_some() {
                return Ok(res
                    .add_attribute("winner", game.winner.unwrap().to_string())
                    .add_message(BankMsg::Send {
                        to_address: info.sender.to_string(),
                        amount: game.prize.clone(),
                    }));
            } else {
                let prize = game.get_half_prize();

                return Ok(res.add_messages(vec![
                    BankMsg::Send {
                        to_address: info.sender.to_string(),
                        amount: prize.clone(),
                    },
                    BankMsg::Send {
                        to_address: opponent,
                        amount: prize.clone(),
                    },
                ]));
            }
        }

        Ok(res)
    }
}
```

Query module reads data from contract state. It has only one method which is Games and receives two optional parameters **key** being the host address and the opponent address and **status** which will query the games with the given status, if none of the optional parameters are submitted the smart contract will return the entire stored data.
> src/contract/query.rs
```rs
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
```

## Tests

The tests tries to achieve the maximum coverage possible by creating the most useful tests under KISS (Keep It Simple, Stupid) approach so you may see some duplicated code in the testing module. This is the latest test coverage achieved with the current version of the module asserting the responses from the smart contract:

```bash
running 28 tests
test test::accept::accept_inexistent ... ok
test test::accept::accept_with_incorrect_coord ... ok
test test::accept::accept_with_incorrect_host ... ok
test test::invite::invite_against_itself ... ok
test test::accept::accept_with_more_funds ... ok
test test::accept::accept_with_different_funds ... ok
test test::accept::accept_with_less_funds ... ok
test test::accept::accept_on_already_played_coords ... ok
test test::invite::invite_when_wrong_coordinate ... ok
test test::invite::invite_when_already_in_progress_game ... ok
test test::play::play_invited_round ... ok
test test::play::play_inexistent_round ... ok
test test::play::play_round_on_existent_symbol ... ok
test test::query_handled_errors::query_by_invalid_opponent ... ok
test test::query_handled_errors::query_by_invalid_host ... ok
test test::query_happy_path::empty_games_with_both_users ... ok
test test::query_happy_path::empty_games_with_no_users ... ok
test test::play::play_round_with_invalid_coords ... ok
test test::play::play_two_rounds_as_opponent ... ok
test test::accept::accept ... ok
test test::invite::invite ... ok
test test::play::play_two_rounds_as_host ... ok
test test::reject::reject_inexistent_existent_round ... ok
test test::play::play_round ... ok
test test::reject::reject_round ... ok
test test::happy_paths::host_wins ... ok
test test::happy_paths::opponent_wins ... ok
test test::happy_paths::tie ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

Aug 01 14:33:25.999  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/contract/execute.rs: 61, 92-93
|| src/contract/query.rs: 48-49, 51-54
|| src/models/state.rs: 87, 90, 116, 149, 183, 223, 228
|| Tested/Total Lines:
|| src/contract/execute.rs: 124/127
|| src/contract/instantiate.rs: 2/2
|| src/contract/query.rs: 25/31
|| src/models/state.rs: 58/65
|| src/test/accept.rs: 193/193
|| src/test/happy_paths.rs: 225/225
|| src/test/invite.rs: 91/91
|| src/test/play.rs: 219/219
|| src/test/query_handled_errors.rs: 28/28
|| src/test/query_happy_path.rs: 24/24
|| src/test/reject.rs: 40/40
98.47% coverage, 1029/1045 lines covered
```