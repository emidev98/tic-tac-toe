import React, { ReactElement, useContext, useEffect } from "react";
import AppConfig, { STORAGE_KEY } from "models/AppConfig";
import { LCDClient } from "@terra-money/terra.js";
import appConfig from '../AppConfig.json';
import { WalletProvider } from '@terra-money/wallet-provider';
import chainOptions from 'networks.json';

type BlockchainProviderType = {
    children: ReactElement
}

type BlockchainContext = {
    lcd: LCDClient,
    networkName: string,
}

export const BlockchainContext = React.createContext<BlockchainContext>({
    lcd: new LCDClient({
        URL: appConfig.URL,
        chainID: appConfig.chainID
    }),
    networkName: appConfig.networkName
});

export const BlockchainProvider = ({ children }: BlockchainProviderType) => {
    let { lcd, networkName } = useContext(BlockchainContext);

    useEffect(() => {
        const storedAppConfig: AppConfig = JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');
        const isValidStoredData = storedAppConfig.storageVersion === appConfig.storageVersion;
        const config: AppConfig = isValidStoredData ? storedAppConfig : appConfig;

        localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
        lcd = new LCDClient({
            URL: config.URL,
            chainID: config.chainID
        });
        networkName = config.networkName;
    }, []);

    return (
        <WalletProvider {...chainOptions}>
            <BlockchainContext.Provider value={{ lcd, networkName}}>
                {children}
            </BlockchainContext.Provider>
        </WalletProvider>
    );
}

export default BlockchainProvider;
