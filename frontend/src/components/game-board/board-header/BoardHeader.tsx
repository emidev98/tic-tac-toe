import React from 'react'
import './BoardHeader.scss'
import { PlayerSymbol } from 'models/PlayerSymbol'

type BoardHeaderProps = {
  playerSymbol: PlayerSymbol,
  onSymbolSelected: (playerSymbol: PlayerSymbol) => void
};

export default function BoardHeader(props: BoardHeaderProps) {
  const { playerSymbol, onSymbolSelected } = props;
  const isSelected = (symbol: PlayerSymbol) => symbol === playerSymbol ? 'selected' : ''

  return (
    <div className='BoardHeader'>
      {playerSymbol ?
        <h3 className='BoardTitle'>Playing as</h3> :
        <h3 className='BoardTitle'>Chose your symbol</h3>
      }
      <div className='BoardOptionsWrapper'>
        <div className={'BoardOption ' + isSelected('X')}
          onClick={() => onSymbolSelected('X')}>X</div>

        <div className={'BoardOption ' + isSelected('O')}
          onClick={() => onSymbolSelected('O')}>O</div>
      </div >
    </div >
  )
}
