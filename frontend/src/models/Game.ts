import { GameStatus } from "./GameStatus";
import { PlayerSymbol } from "./PlayerSymbol";

export interface Game {
    board: Array<Array<PlayerSymbol>>,
    host_symbol: PlayerSymbol,
    player_round: PlayerSymbol,
    prize: [{
        denom: string,
        amount: string
    }],
    status: GameStatus,
    winner?: PlayerSymbol,
}