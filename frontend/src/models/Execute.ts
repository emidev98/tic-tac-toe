import { Coord } from "./Coord"
import { PlayerSymbol } from "./PlayerSymbol"

export type Execute = ExecuteInvite | ExecuteReject | ExecuteAccept | ExecutePlay;

export type ExecuteInvite = {
    invite: {
        coord: Coord,
        host_symbol: PlayerSymbol,
        opponent: String
    }
}

export type ExecuteReject = {
    reject: {
        as_host: boolean,
        opponent: String
    }
}

export type ExecuteAccept = {
    accept: {
        coord: Coord,
        host: String
    }
}

export type ExecutePlay = {
    play: {
        as_host: boolean,
        coord: Coord,
        opponent: String
    }
}