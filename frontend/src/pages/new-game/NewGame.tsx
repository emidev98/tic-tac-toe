import './NewGame.scss';
import React, { createRef, useEffect, useState } from 'react'
import useBlockchain from 'hooks/useBlockchain';
import { GameBoard, GameBoardProps } from 'components/game-board/GameBoard';
import { Coord } from 'models/Coord';
import { PlayerSymbol } from 'models/PlayerSymbol';
import { AddressInput } from 'components/address-input/AddressInput';
import { AmountInput } from 'components/amount-input/AmountInput';
import { useSnackbar } from "notistack";
import { LoadingButton } from '@mui/lab';
import { ExecuteInvite } from 'models/Execute';
import { useNavigate } from 'react-router-dom';
import { AddressHelper } from 'helpers/Address';

export const NewGame = () => {
  const { execute, getConnectedWalletAddress } = useBlockchain();
  const connectedWalletAddress = getConnectedWalletAddress();
  const { enqueueSnackbar } = useSnackbar();
  const navigate = useNavigate()
  const [opponentAddress, setOpponentAddress] = useState<string>('');
  const [amount, setAmount] = useState<string>('');
  const [coord, setCoord] = useState<Coord | undefined>();
  const [loading, setLoading] = useState<boolean>(false);
  const [gameBoard, setGameBoard] = useState<GameBoardProps>({
    data: [
      [undefined, undefined, undefined],
      [undefined, undefined, undefined],
      [undefined, undefined, undefined],
    ],
    headerTitle: "Chose your symbol",
    disabledHeader: false,
    playerSymbol: undefined,
    status: undefined,
    onPlaySelectedPosition: (coord: Coord, playerSymbol: PlayerSymbol) => {
      gameBoard.data[coord.y][coord.x] = playerSymbol;
      gameBoard.playerSymbol = playerSymbol;
      setGameBoard(Object.assign({}, gameBoard));
      setCoord(coord);
    }
  });

  useEffect(()=>{
    if(!connectedWalletAddress) {
      navigate('/games');
      enqueueSnackbar(`Connect your wallet to start a new game`);
    }
  },[connectedWalletAddress]);

  const handleCreateNewGame = async (_event: any) => {
    setLoading(true);
    const req: ExecuteInvite = { invite : {
      coord: coord as Coord,
      host_symbol: gameBoard.playerSymbol,
      opponent: opponentAddress
    }};
    try {
      await execute(req, amount);
      enqueueSnackbar(`Game against '${AddressHelper.parseAddress(opponentAddress)}' created`, {variant: "success"});
      navigate(`/games/${connectedWalletAddress}/${opponentAddress}`);
    }
    catch (e: any) {
      enqueueSnackbar(e.message, {variant: "error"});
    }
    setLoading(false);
  }

  return (
    <div className={`NewGame ${loading ? 'LoadingGame' : ''}`}>
      <AddressInput value={opponentAddress} 
        label='Opponent address'
        onSetValidAddress={setOpponentAddress}/>
      <AmountInput label='Amount (Luna)'
        value={amount}
        minValue={0}
        onSetValidAmount={setAmount}/>
      <GameBoard {...gameBoard}/>
      <LoadingButton className='CreateNewGameButton'
        loading={loading}
        onClick={handleCreateNewGame}
        disabled={!opponentAddress || !gameBoard.playerSymbol || loading}
        variant='outlined'>Create Game</LoadingButton>
    </div>
  )
}
