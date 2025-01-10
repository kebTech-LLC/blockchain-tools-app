type RewardInfo = {
    mint: string;
    vault: string;
    authority: string;
    emissions_per_second_x64: string;
    growth_global_x64: string;
};
  
type TokenInfo = {
    address: string;
    programId: string;
    imageUrl: string;
    name: string;
    symbol: string;
    decimals: number;
    tags: string[];
};
  
export class OrcaPool {
    address: string;
    whirlpoolsConfig: string;
    whirlpoolBump: number[];
    tickSpacing: number;
    tickSpacingSeed: number[];
    feeRate: number;
    protocolFeeRate: number;
    liquidity: string;
    sqrtPrice: string;
    tickCurrentIndex: number;
    protocolFeeOwedA: string;
    protocolFeeOwedB: string;
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
    price: string;
    rewardsUsdc24h: string;
    rewardsUsdc7d: string;
    rewardsUsdc30d: string;
    volumeUsdc24h: string;
    volumeUsdc7d: string;
    volumeUsdc30d: string;
    tvlUsdc: string;
    feesUsdc24h: string;
    feesUsdc7d: string;
    feesUsdc30d: string;
    yieldOverTvl: string;
    rewards: RewardInfo[];
    tokenA: TokenInfo;
    tokenB: TokenInfo;
  
    constructor(data: Partial<OrcaPool>) {
      Object.assign(this, data);
    }
  
    // Add any helper methods if needed
    getLiquidity(): number {
      return parseFloat(this.liquidity);
    }
  
    getPrice(): number {
      return parseFloat(this.price);
    }
  
    getTvlInUsd(): number {
      return parseFloat(this.tvlUsdc);
    }


}
  