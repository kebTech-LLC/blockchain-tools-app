import api from "../server/api";

export class TokenSwap {
    walletKey: string;
    poolAddress: string;
    amount: number;
    amountIsIn: boolean;
    mintOutAddress: string;
    slippageTolerance: number | undefined;

    constructor(walletKey: string, poolAddress: string, amount: number, amountIsIn: boolean, mintOutAddress: string, slippageTolerance?: number) {
        this.walletKey = walletKey;
        this.poolAddress = poolAddress;
        this.amount = amount;
        this.amountIsIn = amountIsIn;
        this.mintOutAddress = mintOutAddress;
        this.slippageTolerance = slippageTolerance;
    }

    toSnakeCase() {
        return {
            wallet_key: this.walletKey,
            pool_address: this.poolAddress,
            amount: this.amount,
            amount_is_in: this.amountIsIn,
            mint_out_address: this.mintOutAddress,
            slippage_tolerance: this.slippageTolerance
        }
    }

    async swap() {
        return await api.poolManager.swapTokens(this);
    }
}
