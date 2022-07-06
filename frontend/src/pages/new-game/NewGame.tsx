import './NewGame.scss';
import React, { createRef, useState } from 'react'
import useBlockchain from 'hooks/useBlockchain';
import { GameBoard, GameBoardProps } from 'components/game-board/GameBoard';
import { Coord } from 'models/Coord';
import { PlayerSymbol } from 'models/PlayerSymbol';

export const NewGame = () => {
  const { execute } = useBlockchain();

  const handleSelectedPosition = (coord: Coord, playerSymbol: PlayerSymbol) => {
    console.log(coord, playerSymbol)
    gameBoard.data[coord.y][coord.x] = playerSymbol;
    gameBoard.playerSymbol = playerSymbol;
    setGameBoard(Object.assign({}, gameBoard));
  };

  const [gameBoard, setGameBoard] = useState<GameBoardProps>({
    data: [
      [undefined, undefined, undefined],
      [undefined, undefined, undefined],
      [undefined, undefined, undefined],
    ],
    playerSymbol: undefined,
    status: undefined,
    onPlaySelectedPosition: handleSelectedPosition
  });


  return (
    <div className='NewGame'>
      <GameBoard {...gameBoard}/>
    </div>
  )
}
