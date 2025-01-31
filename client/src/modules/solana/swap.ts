import api from "../server/api";

export class Swap {
    amount: number;
    amountIsIn: boolean;
    mintInAddress: string;
    mintOutAddress: string;
    slippageTolerance: number | undefined;

    constructor(
        amount: number,
        amountIsIn: boolean,
        mintInAddress: string,
        mintOutAddress: string,
        slippageTolerance: number | undefined
    ) {
        this.amount = amount;
        this.amountIsIn = amountIsIn;
        this.mintInAddress = mintInAddress;
        this.mintOutAddress = mintOutAddress;
        this.slippageTolerance = slippageTolerance;
    }

    toSnakeCase() {
        return {
            amount: this.amount,
            amount_is_in: this.amountIsIn,
            mint_in_address: this.mintInAddress,
            mint_out_address: this.mintOutAddress,
            slippage_tolerance: this.slippageTolerance
        }
    }
    
    async swap() {
        await api.tokens.swap(this);
    }
}

