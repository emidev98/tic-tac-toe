import { Game } from "./Game";
import { GameStatus } from "./GameStatus";

export type Query = {
    host?: string,
    opponent?: string,
    status?: GameStatus,
}

export type QueryResponse = {
    games: Array<Game>,
    host?: String,
    opponent?: String
}