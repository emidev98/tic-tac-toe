import './ReadOnlyGame.scss';
import React, { useEffect, useState } from 'react'
import { Game } from 'models/Game';
import { GameBoard } from 'components/game-board/GameBoard';
import { GameStatus } from 'components/game-status/GameStatus';

type ReadOnlyGameProps = {
  game: Game;
}

export const ReadOnlyGame = (props: ReadOnlyGameProps) => {
  const { game } = props;
  const [gameInProgress, setGameInProgress] = useState<boolean>();
  const [gameCompletedTitle, setGameCompletedTitle] = useState<string>('');

  useEffect(() => {
    const inProgress = game.status === 'INVITED' || game.status === 'PLAYING';
    setGameInProgress(inProgress);
    let message;

    if(game.winner) {
      message = `${game.winner} won! `;
      if(game.prize[0]?.amount !== '0') {
        message += `${(Number(game.prize[0].amount) / 10 ** 6)} Luna sent to its wallet`;
      }
    }
    else if (game.status === 'REJECTED') {
      message = ` Game rejected by ${game.player_round} `;
      if(game.prize[0]?.amount !== '0') {
        message += `${(Number(game.prize[0].amount) / 10 ** 6)} funds returned to ${game.player_round == 'X'? 'O' : 'X'}`;
      }
    }
    else {
      message = ` Tied game! `;
      if(game.prize[0]?.amount !== '0') {
        message += `${(Number(game.prize[0].amount) / 10 ** 6) / 2} sent to each player`;
      }
    }
    setGameCompletedTitle(message);
  }, []);


  return (
    <div className='ReadOnlyGame'>
      {gameInProgress
        ? <>
          <GameStatus status={game.status} />
          <GameBoard disabledHeader
            disabledBoard
            headerTitle='Current player'
            data={game.board}
            playerSymbol={game.player_round}
            status={game.status} />
        </>
        : <>
          <GameBoard disabledHeader
            disabledBoard
            headerTitle={gameCompletedTitle}
            winner={game.winner}
            data={game.board}
            playerSymbol={game.player_round}
            status={game.status} />
        </>}
    </div>
  )
}
