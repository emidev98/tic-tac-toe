import './Game.scss';
import React, { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import useBlockchain from 'hooks/useBlockchain';
import { Game as GameModel } from 'models/Game';
import { GameBoard } from 'components/game-board/GameBoard';
import { PlayerSymbol } from 'models/PlayerSymbol';
import { Coord } from 'models/Coord';
import { LoadingButton } from '@mui/lab';
import { ExecuteAccept, ExecutePlay, ExecuteReject } from 'models/Execute';
import { useSnackbar } from 'notistack';

export const Game = () => {
  const { hostAddress, opponentAddress } = useParams();
  const { query, execute, getConnectedWalletAddress } = useBlockchain();
  const connectedWalletAddress = getConnectedWalletAddress();
  const navigate = useNavigate()
  const { enqueueSnackbar } = useSnackbar();
  const [loading, setLoading] = useState<boolean>(false);
  const [game, setGame] = useState<GameModel | undefined>();
  const [coord, setCoord] = useState<Coord>();
  const [isCurrentPlayerConnected, setCurrentPlayerConnected] = useState<boolean>();

  const init = async () => {
    const { games } = await query({ host: hostAddress, opponent: opponentAddress });
    const isHostRound = games[0].host_symbol === games[0].player_round;

    if (isHostRound) setCurrentPlayerConnected(hostAddress === connectedWalletAddress);
    else setCurrentPlayerConnected(opponentAddress === connectedWalletAddress);

    setGame(games[0]);
  }

  useEffect(() => {
    init();
  }, [connectedWalletAddress]);

  const handleSelectPosition = (coord: Coord, symbol: PlayerSymbol) => {
    if(game && symbol) {
      game.board[coord.y][coord.x] = symbol;
      setGame(Object.assign({}, game));
      setCoord(coord);
    }
  };

  const handleRejectGame = async () => {
    setLoading(true);
    const req: ExecuteReject = { reject : {
      as_host: hostAddress === connectedWalletAddress,
      opponent: hostAddress as String
    }};
    try {
      await execute(req);
      enqueueSnackbar(`Game rejected`, {variant: "success"});
      navigate(`/games`);
    }
    catch (e: any) {
      enqueueSnackbar(e.message, {variant: "error"});
    }
    setLoading(false);
  }

  const handlePlayRound = async () => {
    setLoading(true);
    const req: ExecutePlay = { play : {
      as_host: hostAddress === connectedWalletAddress,
      coord: coord as Coord,
      opponent: (hostAddress === connectedWalletAddress ? opponentAddress as String : hostAddress as String)
    }};
    try {
      await execute(req);
      enqueueSnackbar(`Position played. Waiting for the opponent`, {variant: "success"});
      await init();
    }
    catch (e: any) {
      enqueueSnackbar(e.message, {variant: "error"});
    }
    setLoading(false);
  }

  const handleAcceptGame = async () => {
    setLoading(true);
    const req: ExecuteAccept = { accept : {
      coord: coord as Coord,
      host: hostAddress as String
    }};
    try {
      const amount = (Number(game?.prize[0].amount) / 10 ** 6).toString();
      await execute(req, amount);
      enqueueSnackbar(`Game accepted! Waiting for the opponent`, {variant: "success"});
      await init();
    }
    catch (e: any) {
      enqueueSnackbar(e.message, {variant: "error"});
    }
    setLoading(false);
  }


  return (
    <div className='Game'>
      {game
        ? <>
          <GameBoard disabledHeader
            disabledBoard={!isCurrentPlayerConnected}
            headerTitle={isCurrentPlayerConnected ? 'Go, is your turn!' : `Next player`}
            data={game.board}
            playerSymbol={game.player_round}
            status={game.status}
            onPlaySelectedPosition={handleSelectPosition} />

          {isCurrentPlayerConnected && game.status === 'INVITED' &&
            <>
              <span className='GamePrize'>
                You have been invite to play a game in value of <b> {(Number(game.prize[0].amount) / 10 ** 6)} Luna</b>
              </span>
              <div className='GameFooter'>
                <LoadingButton 
                  onClick={()=> handleRejectGame()}
                  loading={loading}
                  variant='outlined'>
                  Reject
                </LoadingButton>

                <LoadingButton 
                  onClick={()=> handleAcceptGame()}
                  loading={loading}
                  variant='outlined'
                  disabled={!coord}>
                  Accept
                </LoadingButton>
              </div>
            </>
          }

          {isCurrentPlayerConnected && game.status === 'PLAYING' &&
            <>
              <span className='GamePrize'>
                So, who's gonna win the prize <b> {(Number(game.prize[0].amount) / 10 ** 6)} Luna</b> ?
              </span>
              <div className='GameFooter'>
                <LoadingButton 
                  style={{flexGrow: 1}}
                  onClick={()=> handlePlayRound()}
                  loading={loading}
                  variant='outlined'
                  disabled={!coord}>
                  Play
                </LoadingButton>
              </div>
            </>
          }
        </>
        : <div className='GameLoading'>Loading...</div>}
    </div>
  )
}
