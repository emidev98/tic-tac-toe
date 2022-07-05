import { useRoutes } from 'react-router-dom';

import { Game } from "pages/game/Game";
import { Games } from "pages/games/Games";
import { Home } from "pages/home/Home";
import { NewGame } from "pages/new-game/NewGame";

const useNav = () => {
    const menu = [
      {
        name: 'Games',
        path: '/games',
        element: <Games />,
      },
      {
        name: 'New Game',
        path: '/new-game',
        element: <NewGame />,
      },
    ];
  
    const routes = [
      {
        name: '',
        path: '/',
        element: <Home />,
      },
      {
        name: '',
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