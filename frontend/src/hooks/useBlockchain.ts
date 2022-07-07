import { Coin, MsgExecuteContract } from "@terra-money/terra.js";
import { useConnectedWallet } from "@terra-money/wallet-provider";
import { Execute } from "models/Execute";
import { Query, QueryResponse } from "models/Query";
import { BlockchainContext } from "providers/BlockchainProvider";
import { useContext } from "react";
import config from "../refs.terrain.json";

const useBlockchain = () => {
    let { lcd, networkName } = useContext(BlockchainContext);
    const connectedWallet = useConnectedWallet();

    const query = (games: Query): Promise<QueryResponse> => {
        const contractAddress = getContractAddress();

        return lcd.wasm.contractQuery(contractAddress, { games })
    };

    const execute = async (execute: Execute, amount?: string) => {
        const contractAddress = getContractAddress();
        const coins = amount && amount != "0" 
            ? [new Coin('uluna', (Number(amount) * 10 ** 6).toString())] 
            : [];

        if (connectedWallet) {
            const res = await connectedWallet.sign({
                msgs: [
                    new MsgExecuteContract(
                        connectedWallet.walletAddress,
                        contractAddress,
                        execute,
                        coins
                    ),
                ],
            });
            return lcd.tx.broadcast(res.result as any);
        }
        else Promise.reject("Wallet is not connected");
    };

    const getConnectedWalletAddress = () => {
        return connectedWallet?.walletAddress;
    };

    const getContractAddress = () => {
        const NETWORK_NAME = connectedWallet 
            ? connectedWallet.network.name
            : networkName;
            
        // @ts-ignore
        return config[NETWORK_NAME]?.tic_tac_toe?.contractAddresses?.default;
    };

    return {
        query,
        execute,
        getContractAddress,
        getConnectedWalletAddress
    }
};

export default useBlockchain;