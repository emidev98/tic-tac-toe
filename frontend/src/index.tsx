import { createTheme } from '@mui/material/styles';
import App from './App';
import ReactDOM from "react-dom/client";
import './index.scss';
import { ThemeProvider } from '@emotion/react';
import { BlockchainProvider } from 'providers/BlockchainProvider';
import { BrowserRouter } from 'react-router-dom';
import React from 'react';
import { SnackbarProvider } from 'notistack';

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#fff',
    },
  },
});

const root = ReactDOM.createRoot(document.getElementById("root") as Element);

root.render(
  <React.StrictMode>
    <BlockchainProvider>
      <ThemeProvider theme={darkTheme}>
        <SnackbarProvider
          anchorOrigin={{
            vertical: 'bottom',
            horizontal: 'right'
          }}>
          <BrowserRouter>
            <App />
          </BrowserRouter>
        </SnackbarProvider>
      </ThemeProvider>
    </BlockchainProvider>
  </React.StrictMode>
);