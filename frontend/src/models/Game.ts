import { Coins } from "@terra-money/terra.js";
import { GameStatus } from "./GameStatus";
import { PlayerSymbol } from "./PlayerSymbol";

export interface Game {
    board: Array<Array<PlayerSymbol>>,
    host_symbol: PlayerSymbol,
    player_round: PlayerSymbol,
    prize: Coins,
    status: GameStatus,
    winner?: PlayerSymbol,
}