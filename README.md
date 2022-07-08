# Tic Tac Toe

Game developed and deployed to Terra 2.0 blockchain. 

| Network     | Code ID     | Contract Address |
| ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Testnet     |   1893      | [terra1xzx3p5n4e8u2h4s9jcgycz0ye4nj8p3lxs0xq2n7elm5vs3najzqcc6njr](https://finder.terra.money/testnet/address/terra1xzx3p5n4e8u2h4s9jcgycz0ye4nj8p3lxs0xq2n7elm5vs3najzqcc6njr) |

# Smart Contract Architecture

This smart contract is build with 1 query and 4 different executes. To enable the possibility of a permissionless and trustless game it will contain a state machine that will have the following status:

- INVITED: only one game can be in this status at a time per host and opponent pair. This status is achieved by creating a new game and the following possible status are PLAYING or REJECTED.
- PLAYING: only one game can be in this status at a time per host and opponent pair. To achieve this status must mutate from INVITED.
- COMPLETED: multiple games can be in this status but they have to mutate from PLAYING.
- REJECTED: multiple games can be in this status but they have to mutate from INVITE. 

As you may already have noticed this game can only be played by 1 host and 1 opponent at a time (host being the one who created the game) but when a match is completed or rejected new game can be started.

# QueryMsg

The Games query contains three optional parameters (**host**, **opponent** and **status**) which will query the games with the given status, if none of the optional parameters are submitted the smart contract will return the entire stored data.


# ExecuteMsg

- Invite: create a new game if there is no game in status PLAYING or INVITED. 
- Reject: reject a game in status INVITED and return the funds to the player who requested to play.
- AcceptGame: accept a game in status INVITED only when the sent funds match the game prize. The game will change status to PLAYING.
- Play: continues the match of an existing game in status PLAYING following the rules of [Tic Tac Toe](https://en.wikipedia.org/wiki/Tic-tac-toe). This method also checks the status of the game to validate if the game is finished or not. When the games finishes the funds will be transferred to the winner or splitted between the players if tie.

# Tests

The game only contains unit tests with the approach KISS (Keep It Simple, Stupid) so you may see some duplicated code in the testing module. 

This is the last test coverage achieved with the current version of the module asserting all responses from the smart contract:

```bash
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

Jul 08 11:05:34.891  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/contract/execute.rs: 61, 92-93
|| src/contract/query.rs: 54, 60, 65
|| src/models/state.rs: 70, 86, 89, 115, 198, 211, 216
|| Tested/Total Lines:
|| src/contract/execute.rs: 122/125
|| src/contract/instantiate.rs: 2/2
|| src/contract/query.rs: 38/41
|| src/models/state.rs: 51/58
|| src/test/accept.rs: 192/192
|| src/test/integration.rs: 153/153
|| src/test/invite.rs: 90/90
|| src/test/play.rs: 218/218
|| src/test/query_handled_errors.rs: 26/26
|| src/test/query_happy_path.rs: 60/60
|| src/test/reject.rs: 40/40
|| 
98.71% coverage, 992/1005 lines covered
```