import NewPosition from "@/components/PoolManager/NewPosition.vue";
import { poolManager, ticker } from "..";

type RewardInfo = {
    mint: string;
    vault: string;
    authority: string;
    emissions_per_second_x64: string;
    growth_global_x64: string;
};

class TokenInfo {
    address: string;
    programId: string;
    imageUrl: string;
    name: string;
    symbol: string;
    decimals: number;
    tags: string[];

    constructor(data: any) {
        Object.assign(this, data);
    }

    get isStablecoin() {
        return ["USDC", "USDT", "DAI", "USDH", "UXD", "PAI"].includes(this.symbol);
    }
}

// type TokenInfo = {
//     address: string;
//     programId: string;
//     imageUrl: string;
//     name: string;
//     symbol: string;
//     decimals: number;
//     tags: string[];

//     get isStablecoin() {
//         return this.tags.includes('stablecoin');
//     }
// };

export class OrcaPool {
    address: string;
    whirlpoolsConfig: string;
    whirlpoolBump: number[];
    tickSpacing: number;
    tickSpacingSeed: number[];
    feeRate: number;
    protocolFeeRate: number;
    liquidity: number;
    sqrtPrice: number;
    tickCurrentIndex: number;
    protocolFeeOwedA: number;
    protocolFeeOwedB: number;
    tokenMintA: string;
    tokenVaultA: string;
    feeGrowthGlobalA: string;
    tokenMintB: string;
    tokenVaultB: string;
    feeGrowthGlobalB: string;
    rewardLastUpdatedTimestamp: string;
    updatedAt: string;
    updatedSlot: number;
    writeVersion: number;
    risk: number;
    hasRewards: boolean;
    price: number;
    rewardsUsdc24h: number;
    rewardsUsdc7d: number;
    rewardsUsdc30d: number;
    volumeUsdc24h: number;
    volumeUsdc7d: number;
    volumeUsdc30d: number;
    tvlUsdc: number;
    feesUsdc24h: number;
    feesUsdc7d: number;
    feesUsdc30d: number;
    yieldOverTvl: number;
    rewards: RewardInfo[];
    tokenA: TokenInfo;
    tokenB: TokenInfo;

    constructor(data: any) {
        Object.assign(this, data);

        this.liquidity = parseFloat(data.liquidity || '0');
        this.sqrtPrice = parseFloat(data.sqrtPrice || '0');
        this.protocolFeeOwedA = parseFloat(data.protocolFeeOwedA || '0');
        this.protocolFeeOwedB = parseFloat(data.protocolFeeOwedB || '0');
        this.price = parseFloat(data.price || '0');
        this.rewardsUsdc24h = parseFloat(data.rewardsUsdc24h || '0');
        this.rewardsUsdc7d = parseFloat(data.rewardsUsdc7d || '0');
        this.rewardsUsdc30d = parseFloat(data.rewardsUsdc30d || '0');
        this.volumeUsdc24h = parseFloat(data.volumeUsdc24h || '0');
        this.volumeUsdc7d = parseFloat(data.volumeUsdc7d || '0');
        this.volumeUsdc30d = parseFloat(data.volumeUsdc30d || '0');
        this.tvlUsdc = parseFloat(data.tvlUsdc || '0');
        this.feesUsdc24h = parseFloat(data.feesUsdc24h || '0');
        this.feesUsdc7d = parseFloat(data.feesUsdc7d || '0');
        this.feesUsdc30d = parseFloat(data.feesUsdc30d || '0');
        this.yieldOverTvl = parseFloat(data.yieldOverTvl || '0');
        this.tokenA = new TokenInfo(data.tokenA);
        this.tokenB = new TokenInfo(data.tokenB);
    }

    get name() {
        return `${this.tokenA.symbol}/${this.tokenB.symbol}`;
    }

    get tickerPrice() {
        return ticker.prices[this.tokenA.symbol] && this.tokenB.isStablecoin ? ticker.prices[this.tokenA.symbol] : this.price;
    }

}
