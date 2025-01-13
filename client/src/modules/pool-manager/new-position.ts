import { poolManager, solana } from "..";
import { Wallet } from "../solana/wallet";
import { OrcaPool } from "./orca-pool";

enum PoolType {
    Orca,
    Raydium,
    Serum
}

export class NewPosition {
    wallet: Wallet;
    poolType: PoolType;
    rangeLower: number;
    rangeUpper: number;
    percentage: number;
    distribution: number;
    distributionA: number;
    distributionB: number;
    pool: OrcaPool;
    amountA: number;
    amountB: number;
    amountTotal: number;
    dynamicRange: boolean;
    dynamicRangeLower: number;
    dynamicRangeUpper: number;
    manualRangeLower: number;
    manualRangeUpper: number;
    walletBalanceTokenA: number;
    walletBalanceTokenB: number;
    walletBalanceTotal: number;

    constructor(pool: OrcaPool) {
        this.wallet = solana.programmaticWallet!;
        this.poolType = PoolType.Orca;
        this.rangeLower = 0;
        this.rangeUpper = 0;
        this.percentage = 1;
        this.distribution = 50;
        this.distributionA = 50;
        this.distributionB = 50;
        this.pool = pool;
        this.amountTotal = 0;
        this.dynamicRange = true;
        this.dynamicRangeLower = 0;
        this.dynamicRangeUpper = 0;
        this.manualRangeLower = 0;
        this.manualRangeUpper = 0;
        this.walletBalanceTokenA = 0;
        this.walletBalanceTokenB = 0;
        this.walletBalanceTotal = 0;
        this.adjustPercentage(1);
        this.calculateWalletBalance();
       
    }

    open() {
        poolManager.openPosition(this);
    }

    setDynamicRange(dynamic: boolean = true) {
        this.dynamicRange = dynamic;
        if (!dynamic) {
            this.manualRangeLower = this.dynamicRangeLower;
            this.manualRangeUpper = this.dynamicRangeUpper;
        }
    }

    adjustPercentage(percentage: number) {
        this.percentage = percentage / 100;
    }

    calculateWalletBalance() {
        this.wallet.getTokenBalance(this.pool.tokenMintA)
            .then(balance => {
                this.walletBalanceTokenA = balance;
                this.wallet.getTokenBalance(this.pool.tokenMintB)
                    .then(balance => {
                        this.walletBalanceTokenB = balance;
                        this.walletBalanceTotal = this.walletBalanceTokenA + this.walletBalanceTokenB;
                        this.amountTotal = this.walletBalanceTotal;
                    });
            });
    }

    calculateDynamicRange() {
        const factor = this.pool.tickerPrice * this.percentage;
        this.dynamicRangeLower = (this.pool.tickerPrice - factor);
        this.dynamicRangeUpper = (this.pool.tickerPrice + factor);
    }

    recalculate() {
        if (this.dynamicRange) {
            this.calculateDynamicRange()
            this.rangeLower = this.dynamicRangeLower;
            this.rangeUpper = this.dynamicRangeUpper;
        } else {
            this.rangeLower = this.manualRangeLower;
            this.rangeUpper = this.manualRangeUpper;
            this.distribution = (this.pool.tickerPrice - this.rangeLower) / (this.rangeUpper - this.rangeLower) * 100;
            this.percentage = (this.rangeUpper - this.rangeLower) / this.pool.tickerPrice;
        }

        this.distributionA = Math.max(0, Math.min(100, this.distribution));
        this.distributionB = Math.max(0, Math.min(100, 100 - this.distribution));

        this.amountA = this.amountTotal * this.distributionA / 100;
        this.amountB = this.amountTotal * this.distributionB / 100;

        this.rangeLower = this.distributionA

        console.log(this);
    }

    reset() {
        const pool = this.pool;
        poolManager.closeNewPosition();
        poolManager.setupNewPosition(pool);
    }
}