import { MsgExecuteContract } from "@terra-money/terra.js";
import { useConnectedWallet } from "@terra-money/wallet-provider";
import { Execute } from "models/Execute";
import { Query, QueryResponse } from "models/Query";
import { BlockchainContext } from "providers/BlockchainProvider";
import { useContext } from "react";
import config from "../refs.terrain.json";

const useBlockchain = () => {
    let { lcd } = useContext(BlockchainContext);
    const connectedWallet = useConnectedWallet();

    const query = (games: Query): Promise<QueryResponse> => {
        const contractAddress = getContractAddress();

        return lcd.wasm.contractQuery(contractAddress, { games })
    };

    const execute = (execute: Execute) => {
        const contractAddress = getContractAddress();

        if (connectedWallet) {
            return connectedWallet.post({
                msgs: [
                    new MsgExecuteContract(
                        connectedWallet.walletAddress,
                        contractAddress,
                        execute
                    ),
                ],
            });
        }
        else Promise.reject("Wallet is not connected");
    };

    const getContractAddress = () => {
        // @ts-ignore
        return config[lcd.network.name]?.tic_tac_toe?.contractAddresses?.default;
    };

    return {
        query,
        execute,
        getContractAddress
    }
};

export default useBlockchain;