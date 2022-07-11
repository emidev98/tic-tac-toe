export class AddressHelper {
    private constructor () { }

    public static parseAddress = (address: string | undefined) => {
        if (!address) return '';

        return '...' + address.slice(-6, address.length);
    };

    public static parseGameAddress = (
        hostAddress: string | undefined, 
        opponentAddress: string | undefined
    ) => {
        if (!hostAddress || !opponentAddress) return '';

        return this.parseAddress(hostAddress) + "/" + this.parseAddress(opponentAddress);
    };
}