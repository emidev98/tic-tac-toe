import { Button, TableCell } from '@mui/material';
import { GameBoard } from 'components/game-board/GameBoard';
import { QueryMatch } from 'models/Query';
import React, { useEffect, useState } from 'react';
import './GameTableRow.scss';
import EmailIcon from '@mui/icons-material/Email';
import CloseIcon from '@mui/icons-material/Close';
import DoneIcon from '@mui/icons-material/Done';
import RotateLeftIcon from '@mui/icons-material/RotateLeft';
import ArrowForwardIosIcon from '@mui/icons-material/ArrowForwardIos';

type GameTableRowProps = {
    index: number,
    data: QueryMatch,
    onGoToDetails: (match: QueryMatch) => void
}

export const GameTableRow = (props: GameTableRowProps) => {
    const { game, host, opponent } = props.data;

    const parseAddress = (address: string | undefined) => {
        if (!address) return '';

        return '...' + address.slice(-8, address.length - 1);
    };

    return (
        <>
            <TableCell className='GameRowStatusCell'>
                <div>
                    {game.status === 'INVITED' && <EmailIcon />}
                    {game.status === 'COMPLETED' && <DoneIcon />}
                    {game.status === 'PLAYING' && <RotateLeftIcon />}
                    {game.status === 'REJECTED' && <CloseIcon />}
                    <span>{game.status}</span>
                </div>
            </TableCell>
            <TableCell>
                <span>{parseAddress(host)}</span> / <span>{parseAddress(opponent)}</span>
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
            <TableCell>
                <Button variant='outlined'
                    endIcon={<ArrowForwardIosIcon />}
                    onClick={()=> props.onGoToDetails(props.data)}>DETAILS</Button>
            </TableCell>
        </>
    )
}
