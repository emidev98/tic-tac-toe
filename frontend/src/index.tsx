import { createTheme } from '@mui/material/styles';
import App from './App';
import ReactDOM from "react-dom/client";
import './index.scss';
import { ThemeProvider } from '@emotion/react';
import { BlockchainProvider } from 'providers/BlockchainProvider';
import { WalletProvider } from '@terra-money/wallet-provider';
import chainOptions from 'networks.json';
import { BrowserRouter } from 'react-router-dom';
import React from 'react';

const root = ReactDOM.createRoot(document.getElementById("root") as Element);

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#fff',
    },
  },
});

root.render(
  <React.StrictMode>
    <WalletProvider {...chainOptions}>
      <BlockchainProvider>
        <ThemeProvider theme={darkTheme}>
          <BrowserRouter>
            <App />
          </BrowserRouter>
        </ThemeProvider>
      </BlockchainProvider>
    </WalletProvider>
  </React.StrictMode>
);