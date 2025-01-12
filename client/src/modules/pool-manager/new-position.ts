import { poolManager, wallets } from "..";
import { OrcaPool } from "./orca-pool";

enum PoolType {
    Orca,
    Raydium,
    Serum
}

export class NewPosition {
    walletKey: string;
    walletType: string;
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

    constructor(pool: OrcaPool) {
        this.walletKey = wallets.solanaWalletManager.programmaticWalletKey?.toString() || '';
        this.walletType = wallets.solanaWalletManager.getWalletType(this.walletKey) || '';
        this.poolType = PoolType.Orca;
        this.rangeLower = 0;
        this.rangeUpper = 0;
        this.percentage = 1;
        this.distribution = 50;
        this.distributionA = 50;
        this.distributionB = 50;
        this.pool = pool;
        this.amountTotal = 1000;
        this.dynamicRange = true;
        this.dynamicRangeLower = 0;
        this.dynamicRangeUpper = 0;
        this.manualRangeLower = 0;
        this.manualRangeUpper = 0;
        this.adjustPercentage(1);
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
        }
        this.distributionA = this.distribution;
        this.distributionB = 100 - this.distribution;
        console.log(this)
    }
}