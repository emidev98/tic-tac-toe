import { getChainOptions, WalletProvider } from '@terra-money/wallet-provider';
import App from './App';
import ReactDOM from "react-dom/client";
import './index.css';

const root = ReactDOM.createRoot(document.getElementById("root") as Element);

getChainOptions().then((chainOptions) => {
  root.render(
    <WalletProvider {...chainOptions}>
      <App />
    </WalletProvider>
  );
});
