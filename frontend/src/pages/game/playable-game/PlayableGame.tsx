import { LoadingButton } from '@mui/lab';
import { GameBoard } from 'components/game-board/GameBoard';
import { Coord } from 'models/Coord';
import { Game } from 'models/Game';
import { PlayerSymbol } from 'models/PlayerSymbol';
import React, { useState } from 'react'
import './PlayableGame.scss';

type PlayableGameProps = {
    game: Game,
    onRejectGame: () => Promise<any>,
    onPlayGame: (coord: Coord) => Promise<any>
}


export const PlayableGame = (props: PlayableGameProps) => {
    const [game, setGame] = useState(props.game);
    const [coord, setCoord] = useState<Coord | undefined>();
    const [isRejectGameLoading, setRejectGameLoading] = useState<boolean>(false);
    const [isPlayGameLoading, setPlayGameLoading] = useState<boolean>(false);

    const handlePlaySelectedPosition = (coord: Coord, playerSymbol: PlayerSymbol) => {
        game.board[coord.y][coord.x] = playerSymbol;
        setGame(Object.assign({}, game));
        setCoord(coord);
    };

    const handleRejectGame = async () => {
        setRejectGameLoading(true);
        await props.onRejectGame();
        setRejectGameLoading(false);
    };

    const handlePlayGame = async () => {
        setPlayGameLoading(true);
        await props.onPlayGame(coord as Coord);
        setPlayGameLoading(false);
    };

    return (
        <div className={`PlayableGame ${game.status === 'PLAYING' ? 'PlayableGameAccepted' : ''}`}> 
            <GameBoard disabledHeader
                headerTitle={game.status === 'INVITED' ? 'You have been invited to play' : 'Your turn to play'}
                data={game.board}
                status={game.status}
                playerSymbol={game.player_round}
                onPlaySelectedPosition={handlePlaySelectedPosition} />

            <div className='PlayableGameFooter'>
                {game.status === 'INVITED' && (
                    <LoadingButton
                        onClick={() => handleRejectGame()}
                        loading={isRejectGameLoading}
                        variant='outlined'>
                        Reject
                    </LoadingButton>)}

                <LoadingButton
                    onClick={() => handlePlayGame()}
                    loading={isPlayGameLoading}
                    variant='outlined'
                    disabled={!coord}>
                    {game.status === 'INVITED' ? 'Accept' : 'Play'}
                </LoadingButton>
            </div>
        </div>
    )
}
