import { Button } from '@mui/material';
import { useWallet, WalletStatus } from '@terra-money/wallet-provider';

export const ConnectWallet = () => {
  const {
    status,
    availableConnectTypes,
    availableConnections,
    connect,
    disconnect,
  } = useWallet();

  return (
    <div>
      {status === WalletStatus.WALLET_NOT_CONNECTED && (
        <>
          {availableConnections.map(({ type, name, icon }) => (
            <Button key={'connection-' + type}
              onClick={() => connect(type)}>
              <img src={icon} alt={name} style={{ width: '1em', height: '1em' }} />
              <span>{name}</span>
            </Button>
          ),
          )}
        </>
      )}

      {status === WalletStatus.WALLET_CONNECTED && (
        <Button onClick={() => disconnect()}>Disconnect</Button>
      )}
    </div>
  );
}
