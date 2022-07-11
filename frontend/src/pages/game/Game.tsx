import './Game.scss';
import React, { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import useBlockchain from 'hooks/useBlockchain';
import { Game as GameModel } from 'models/Game';
import { Coord } from 'models/Coord';
import { ExecuteAccept, ExecutePlay, ExecuteReject } from 'models/Execute';
import { useSnackbar } from 'notistack';
import { AddressHelper } from 'helpers/Address';
import { ReadOnlyGame } from './read-only-game/ReadOnlyGame';
import { PlayableGame } from './playable-game/PlayableGame';

export const Game = () => {
  const { hostAddress, opponentAddress } = useParams();
  const { query, execute, getConnectedWalletAddress } = useBlockchain();
  const connectedWalletAddress = getConnectedWalletAddress();
  const navigate = useNavigate()
  const { enqueueSnackbar } = useSnackbar();
  const [loading, setLoading] = useState<boolean>(false);
  const [game, setGame] = useState<GameModel | undefined>();
  const [isReadOnly, setReadOnly] = useState<boolean>();

  const init = async () => {
    const res = await query({
      key: {
        host: hostAddress as string,
        opponent: opponentAddress as string
      }
    });

    if (res.length === 0) {
      navigate('/');
      const message = `Cannot find game '${AddressHelper.parseGameAddress(hostAddress, opponentAddress)}' on current network`;
      return enqueueSnackbar(message, { variant: "error" });
    }

    const match = res[0].game;
    setViewType(match);
    setGame(Object.assign({}, match));
  }

  const setViewType = (match: GameModel) => {
    const isGameFinished = match.status === 'REJECTED' || match.status === 'COMPLETED';

    if (isGameFinished) {
      return setReadOnly(true);
    }

    const isHostRound = match.host_symbol === match.player_round;
    const isHostConnected = hostAddress === connectedWalletAddress;
    const isOpponentConnected = opponentAddress === connectedWalletAddress;

    if (isHostRound && isHostConnected) setReadOnly(false);
    else if (!isHostRound && isOpponentConnected) setReadOnly(false);
    else setReadOnly(true);
  }

  useEffect(() => {
    init();
  }, [connectedWalletAddress, hostAddress, opponentAddress]);

  const handleRejectGame = async () => {
    setLoading(true);
    const req: ExecuteReject = {
      reject: {
        as_host: hostAddress === connectedWalletAddress,
        opponent: hostAddress as String
      }
    };
    try {
      await execute(req);
      enqueueSnackbar(`Game rejected`, { variant: "success" });
      navigate(`/games`);
    }
    catch (e: any) {
      enqueueSnackbar(e.message, { variant: "error" });
    }
    setLoading(false);
  }

  const handlePlayGame = async (coord: Coord) => {
    setLoading(true);

    try {
      if (game?.status === 'INVITED') {
        const req: ExecuteAccept = {
          accept: {
            coord: coord,
            host: hostAddress as String
          }
        };
        const amount = (Number(game?.prize[0].amount) / 10 ** 6).toString();
        await execute(req, amount);
        enqueueSnackbar(`Game accepted! Waiting for the opponent`, { variant: "success" });
      }
      else {
        const req: ExecutePlay = {
          play: {
            as_host: hostAddress === connectedWalletAddress,
            coord: coord,
            opponent: (hostAddress === connectedWalletAddress ? opponentAddress as String : hostAddress as String)
          }
        };
        await execute(req);
        enqueueSnackbar(`Position played. Waiting for the opponent`, { variant: "success" });
      }
    }
    catch (e: any) {
      enqueueSnackbar(e.message, { variant: "error" });
    }
    await init();
    setLoading(false);
  }

  return (
    <div className={`Game ${loading ? 'LoadingGame' : ''}`}>
      {game &&
        <>
          {isReadOnly
            ? <ReadOnlyGame game={game} />
            : <PlayableGame game={game} onPlayGame={handlePlayGame} onRejectGame={handleRejectGame} />}
        </>
      }
    </div>
  )
}
