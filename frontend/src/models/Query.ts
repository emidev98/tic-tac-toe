import { Game } from "./Game";
import { GameStatus } from "./GameStatus";

export type Query = {
    key?: QueryKey,
    status?: GameStatus,
}

export type QueryKey = {
    host: string,
    opponent: string,
};

export type QueryResponse = Array<QueryMatch>;

export type QueryMatch = {
    game: Game,
    host?: string,
    opponent?: string
};