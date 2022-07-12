import React from 'react'
import './GameStatus.scss';
import { GameStatus as GameStatusType } from '../../models/GameStatus';
import EmailIcon from '@mui/icons-material/Email';
import CloseIcon from '@mui/icons-material/Close';
import DoneIcon from '@mui/icons-material/Done';
import RotateLeftIcon from '@mui/icons-material/RotateLeft';

type GameStatusProps = {
    status: GameStatusType
};

export const GameStatus = (props: GameStatusProps) => {
    return (
        <div className='GameStatus'>
            {props.status === 'INVITED' && <EmailIcon />}
            {props.status === 'COMPLETED' && <DoneIcon />}
            {props.status === 'PLAYING' && <RotateLeftIcon />}
            {props.status === 'REJECTED' && <CloseIcon />}
            <span className='GameStatusText'>{props.status}</span>
        </div>
    )
}
