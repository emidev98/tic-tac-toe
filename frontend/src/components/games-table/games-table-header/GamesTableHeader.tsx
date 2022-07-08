import React from 'react'
import './GamesTableHeader.scss';
import { TableRow } from '@mui/material';
import TableCell from '@mui/material/TableCell'

const TABLE_HEADER = [{
    name: 'Status',
    className: 'StatusCell'
},{
    name: 'Players',
    className: 'PlayersCell'
},{
    name: 'Prize',
    className: 'PrizeCell'
},{
    name: 'Game preview',
    className: 'GamePreviewCell'
},{
    name: '',
    className: 'GameActionsCell'
}];

export const GamesTableHeader = () => {

  return (
    <TableRow className='GamesTableHeader'>
        {TABLE_HEADER.map((header, index) => (
            <TableCell key={index} className={header.className}>{header.name}</TableCell>
        ))}
    </TableRow>
  )
}
