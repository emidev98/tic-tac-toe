import './WalletController.scss';
import React, { useState } from 'react';
import { Button, Menu, MenuItem } from '@mui/material';
import { ConnectType, useWallet, WalletStatus } from '@terra-money/wallet-provider';
import { AddressHelper } from 'helpers/Address';

type WalletControllerProps = {
  className?: string
}

export const WalletController = ({ className }: WalletControllerProps) => {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);
  const {
    status,
    wallets,
    availableConnections,
    connect,
    disconnect,
  } = useWallet();

  const handleOpenMenu = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleCloseMenu = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(null);
  };

  const handleSelectedOption = (connectType: ConnectType) => {
    connect(connectType);
    setAnchorEl(null);
  };

  return (
    <div className={'WalletController ' + className}>
      {status !== WalletStatus.WALLET_CONNECTED && (
        <Button variant="outlined"
          onClick={handleOpenMenu}>
          <span>Connect</span>
        </Button>
      )}

      {status === WalletStatus.WALLET_CONNECTED && (
        <Button variant="outlined"
          onClick={() => disconnect()}>
          <span className='WalletAddress'>
            {AddressHelper.parseAddress(wallets[0].terraAddress)}
            </span>
        </Button>
      )}

      <Menu id="WalletControllerMenu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleCloseMenu}>
        {availableConnections.map(({ type, name, icon }) => (
          <MenuItem key={type} onClick={() => handleSelectedOption(type)}>
            <img src={icon} alt={name} />
            <span>{name}</span>
          </MenuItem>
        ))}
      </Menu>
    </div>
  );
}
