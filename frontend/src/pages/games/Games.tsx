import './Games.scss';
import React, { useEffect, useState } from 'react'
import { QueryMatch, QueryResponse } from 'models/Query';
import useBlockchain from 'hooks/useBlockchain'
import { GamesTable } from 'components/games-table/GamesTable';
import { useNavigate } from 'react-router-dom';

export const Games = () => {
  const { query } = useBlockchain();
  const navigate = useNavigate()
  const [gamesResponse, setGamesResponse] = useState<QueryResponse>();

  useEffect(() => {
    const init = async () => {
      const response = await query({});
      setGamesResponse(response);
    }
    init();
  }, []);

  const handleGoToDetails = (game: QueryMatch) => {
    navigate(`/games/${game.host}/${game.opponent}`);
  };

  return (
    <div className='Games'>
      {gamesResponse?.length
        ? <GamesTable data={gamesResponse} onGoToDetails={handleGoToDetails}/> 
        : <div>Loading...</div>}
    </div>
  )
}
