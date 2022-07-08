import { QueryMatch } from 'models/Query';
import React from 'react';
import './GamesTable.scss';
import { TableVirtuoso } from 'react-virtuoso'

import Table from '@mui/material/Table'
import TableBody from '@mui/material/TableBody'
import { GamesTableHeader } from './games-table-header/GamesTableHeader';
import { GameTableRow } from './game-table-row/GameTableRow';

type GamesTableProps = {
    data: Array<QueryMatch>;
    onGoToDetails: (match: QueryMatch) => void;
}

export const GamesTable = (props: GamesTableProps) => {
    const { data, onGoToDetails} = props;

    return (
        <TableVirtuoso 
            className='GamesTable'
            data={data}
            components={{
                Table: (props) => <Table {...props} style={{ borderCollapse: 'separate' }} />,
                TableBody: React.forwardRef((props, ref) => <TableBody {...props} ref={ref} />),
            }}
            fixedHeaderContent={GamesTableHeader}
            itemContent={(index, data)=> <GameTableRow index={index} data={data} onGoToDetails={onGoToDetails}/>}
        />
    )
}
