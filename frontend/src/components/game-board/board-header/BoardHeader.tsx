import React from 'react'
import './BoardHeader.scss'
import { PlayerSymbol } from 'models/PlayerSymbol'
import { GameStatus } from 'models/GameStatus';

type BoardHeaderProps = {
  playerSymbol: PlayerSymbol,
  title?: string,
  disabled?: boolean,
  gameStatus?: GameStatus,
  onSymbolSelected: (playerSymbol: PlayerSymbol) => void
};

export default function BoardHeader(props: BoardHeaderProps) {
  const { playerSymbol, onSymbolSelected, disabled, gameStatus } = props;
  const isSelected = (symbol: PlayerSymbol) => symbol === playerSymbol ? 'selected' : ''

  return (
    <div className={`BoardHeader ${disabled && 'DisabledHeader'}`}>
      {props.title && 
        <h3 className='BoardTitle'>{props.title}</h3>
      }

      {(gameStatus === 'INVITED' || gameStatus === 'PLAYING') && <>
        <div className='BoardOptionsWrapper'>
          <div className={'BoardOption ' + isSelected('X')}
            onClick={() => onSymbolSelected('X')}>
            X
          </div>

          <div className={'BoardOption ' + isSelected('O')}
            onClick={() => onSymbolSelected('O')}>
            O
          </div>
        </div>
      </>}
    </div >
  )
}
