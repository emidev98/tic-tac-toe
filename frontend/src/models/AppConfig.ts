export const STORAGE_KEY = "TTT_STORAGE";

type AppConfig = {
    storageVersion: number;
    networkName: string;
    URL: string;
    chainID: string;
    isConnectedWallet: boolean;
};

export default AppConfig;