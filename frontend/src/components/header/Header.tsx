import './Header.scss';
import React from 'react'
import { NavLink } from 'react-router-dom';
import { useWallet, WalletStatus } from '@terra-money/wallet-provider';
import { WalletController } from './WalletController/WalletController';

type HeaderProps = {
  menu: any;
}

export const Header = ({ menu }: HeaderProps) => {
  const { status } = useWallet();

  return (
    <div className='Header'>
      <header className='Menu'>
        {menu.map((menuEntry: any, index: number) => (
          <NavLink className='MenuEntry'
            style={menuEntry.hidden ? { display: 'none' } : {}}
            key={index}
            to={menuEntry.path}>
            {menuEntry.icon && <img src={menuEntry.icon} alt="icon" />}
            {menuEntry.name && <span>{menuEntry.name}</span>}
          </NavLink>
        ))}

        <WalletController />
      </header>
    </div>
  )
}
