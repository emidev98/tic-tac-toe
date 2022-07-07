import React, { RefObject, useState } from 'react';
import './GameBoard.scss';
import { GameStatus } from 'models/GameStatus';
import { PlayerSymbol } from 'models/PlayerSymbol';
import { Coord } from 'models/Coord';
import BoardHeader from './board-header/BoardHeader';

export type GameBoardProps = {
    data: Array<Array<PlayerSymbol>>,
    playerSymbol: PlayerSymbol,
    status: GameStatus,
    headerTitle?: string,
    hideHeader?: boolean,
    disabledHeader?: boolean,
    disabledBoard?: boolean,
    onPlaySelectedPosition?: (coord: Coord, playerSymbol: PlayerSymbol) => void,
};

export const GameBoard = (props: GameBoardProps) => {
    const { data, playerSymbol, disabledBoard } = props;
    const [symbol, setSymbol] = useState(playerSymbol);

    const handleTryPlay = (rowIndex: number, cellIndex: number) => {
        if (symbol && props.onPlaySelectedPosition) {
            const coord = { x: cellIndex, y: rowIndex };
            props.onPlaySelectedPosition(coord, symbol);
        }
    }

    return (
        <div className='GameBoard'>
            {!props.hideHeader &&
                <BoardHeader
                    title={props.headerTitle}
                    disabled={props.disabledHeader}
                    playerSymbol={symbol}
                    onSymbolSelected={setSymbol} />
            }

            <div className={`GameBody ${(disabledBoard) && 'DisabledGameBoard'} ${!symbol && 'NoSymbolSelected'}`}>
                {data.map((row, rowIndex) => (
                    <div className={`GameRow Row${rowIndex}`} key={rowIndex}>

                        {row.map((playedSymbol, cellIndex) => (
                            <div className={`GameCell Cell${cellIndex} ${playedSymbol && 'GameCellWithData'}`}
                                key={`${rowIndex}${cellIndex}`}
                                onClick={() => handleTryPlay(rowIndex, cellIndex)}>
                                {playedSymbol}
                            </div>
                        ))}

                    </div>
                ))}
            </div>
        </div>
    )
}
