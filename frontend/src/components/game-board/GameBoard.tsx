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
    onPlaySelectedPosition: (coord: Coord, playerSymbol: PlayerSymbol) => void,
};

export const GameBoard = (props: GameBoardProps) => {
    const { data, onPlaySelectedPosition, playerSymbol } = props;
    const [symbol, setSymbol] = useState(playerSymbol);

    const handleTryPlay = (rowIndex: number, cellIndex: number) => {
        const selectedCoord = data[rowIndex][cellIndex];
        console.log("selectedCoord ",selectedCoord)
        
        if(symbol) onPlaySelectedPosition({ y: rowIndex, x: cellIndex }, symbol);
    }

    return (
        <div className='GameBoard'>
            <BoardHeader 
                playerSymbol={symbol}
                onSymbolSelected={setSymbol} />

            <div className={`GameBody ${!symbol && 'without-symbol'}`}>
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
