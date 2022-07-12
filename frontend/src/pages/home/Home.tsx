import './Home.scss';
import React from 'react'
import { Copyright, GitHub, InfoSharp } from '@mui/icons-material';
import { useConnectedWallet } from '@terra-money/wallet-provider';

export const Home = () => {
  const walletConnected = useConnectedWallet();

  return (
    <div className='Home'>
      <div className='HomeHeader'>
        {!walletConnected
          ? <h3><span>Hey!</span> Connect your wallet to <span>Terra 2.0</span> testnet and play!</h3>
          : <h3>Now that your wallet is connected create a <span>new game</span>!</h3>}

      </div>

      <h3>...</h3>

      <ul className='HomeRules'>
        <li>The invitation can have a prize which the opponent has to match</li>
        <li>When someone invites you to a game it can be can be rejected</li>
        <li>One game can be in progress at the time against the same address</li>
        <li>In case of tie the funds will be send to the participants</li>
      </ul>

      <div className='HomeFooter'>
        <a href='https://github.com/emidev98/tic-tac-toe' target='_blank'>
          <GitHub />
          <span>Code</span>
        </a>
        <a href='https://en.wikipedia.org/wiki/Tic-tac-toe' target='_blank'>
          <InfoSharp />
          <span>Wikipedia</span>
        </a>
        <a href='https://github.com/emidev98/tic-tac-toe/blob/main/LICENSE.md' target='_blank'>
          <Copyright />
          <span>License</span>
        </a>
      </div>
    </div>
  )
}
