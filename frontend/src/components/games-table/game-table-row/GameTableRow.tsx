import { Button, TableCell } from '@mui/material';
import { GameBoard } from 'components/game-board/GameBoard';
import { QueryMatch } from 'models/Query';
import React, { useEffect, useState } from 'react';
import './GameTableRow.scss';
import ArrowForwardIosIcon from '@mui/icons-material/ArrowForwardIos';
import { AddressHelper } from 'helpers/Address';
import { GameStatus } from 'components/game-status/GameStatus';

type GameTableRowProps = {
    index: number,
    data: QueryMatch,
    onGoToDetails: (match: QueryMatch) => void
}

export const GameTableRow = (props: GameTableRowProps) => {
    const { game, host, opponent } = props.data;

    return (
        <>
            <TableCell className='GameRowStatusCell'>
                <GameStatus status={game.status}/>
            </TableCell>
            <TableCell className='GameRowPlayersCell'>
                <span>{AddressHelper.parseGameAddress(host, opponent)}</span>
            </TableCell>
            <TableCell>
                {(Number(game.prize[0].amount) / 10 ** 6)} Luna
            </TableCell>
            <TableCell className='GameRowPreviewCell'>
                <GameBoard data={game.board}
                    playerSymbol={game.player_round}
                    hideHeader
                    disabledBoard
                    small />
            </TableCell>
            <TableCell className='GameRowDetailsCell'>
                <Button variant='outlined'
                    onClick={()=> props.onGoToDetails(props.data)}>
                        <span className='GameRowButtonText'>DETAILS</span>
                        <ArrowForwardIosIcon />
                    </Button>
            </TableCell>
        </>
    )
}
