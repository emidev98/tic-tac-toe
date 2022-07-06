import { RouteObject, useRoutes } from 'react-router-dom';

import { Game } from "pages/game/Game";
import { Games } from "pages/games/Games";
import { Home } from "pages/home/Home";
import { NewGame } from "pages/new-game/NewGame";

import logo from './assets/logo.svg';
import { useWallet, WalletStatus } from '@terra-money/wallet-provider';
import MenuEntry from 'models/MenuEntry';

const useNav = () => {
  const { status } = useWallet();

  const menu: MenuEntry[] = [
    {
      icon: logo,
      path: '/',
      element: <Home />,
    },
    {
      name: 'Games',
      path: '/games',
      element: <Games />,
    },
    {
      name: 'New Game',
      path: '/new-game',
      element: <NewGame />,
      hidden: status !== WalletStatus.WALLET_CONNECTED,
    },
  ];

  const routes = [
    {
      path: '/game/:id',
      element: <Game />,
    },

    ...menu,
  ];

  return {
    menu,
    element: useRoutes(routes),
  };
};

export default useNav;