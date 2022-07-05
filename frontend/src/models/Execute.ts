import { Coord } from "./Coord"
import { PlayerSymbol } from "./PlayerSymbol"

export type Execute = Invite | Reject | Accept | Play;

export type Invite = {
    coord: Coord,
    host_symbol: PlayerSymbol,
    opponent: String
}

export type Reject = {
    as_host: boolean,
    opponent: String
}

export type Accept = {
    coord: Coord,
    host: String
}

export type Play = {
    as_host: boolean,
    coord: Coord,
    opponent: String
}