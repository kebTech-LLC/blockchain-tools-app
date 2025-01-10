import { ticker } from "..";

export class ManagedPosition {
    address: string;
    balanceTokenA: number;
    balanceTokenAPercentage: number;
    balanceTokenAUsd: number;
    balanceTokenB: number;
    balanceTokenBPercentage: number;
    balanceTokenBUsd: number;
    balanceTotalUsd: number;
    createdAt: Date;
    currentPrice: number;
    inRange: boolean;
    poolAddress: string;
    poolType: string;
    positionMint: string;
    rangeLower: number;
    rangeUpper: number;
    rewardInfos: { amountOwed: number; growthInsideCheckpoint: number }[];
    rewardsOwed: number[];
    tokenA: {
        address: string;
        decimals: number;
        isStablecoin: boolean;
        name: string;
        symbol: string;
    };
    tokenB: {
        address: string;
        decimals: number;
        isStablecoin: boolean;
        name: string;
        symbol: string;
    };
    updatedAt: Date;
    yieldTokenA: number;
    yieldTokenAUsd: number;
    yieldTokenB: number;
    yieldTokenBUsd: number;
    yieldTotalUsd: number;
  
    constructor(data: any) {
        this.address = data.address;
        this.balanceTokenA = data.balance_token_a;
        this.balanceTokenAPercentage = data.balance_token_a_percentage;
        this.balanceTokenAUsd = data.balance_token_a_usd;
        this.balanceTokenB = data.balance_token_b;
        this.balanceTokenBPercentage = data.balance_token_b_percentage;
        this.balanceTokenBUsd = data.balance_token_b_usd;
        this.balanceTotalUsd = data.balance_total_usd;
        this.createdAt = new Date(data.created_at);
        this.currentPrice = data.current_price;
        this.inRange = data.in_range;
        this.poolAddress = data.pool_address;
        this.poolType = data.pool_type;
        this.positionMint = data.position_mint;
        this.rangeLower = data.range_lower;
        this.rangeUpper = data.range_upper;
        this.rewardInfos = data.reward_infos.map((info: any) => ({
            amountOwed: info.amount_owed,
            growthInsideCheckpoint: info.growth_inside_checkpoint,
        }));
        this.rewardsOwed = data.rewards_owed;
        this.tokenA = {
            address: data.token_a.address,
            decimals: data.token_a.decimals,
            isStablecoin: data.token_a.is_stablecoin,
            name: data.token_a.name,
            symbol: data.token_a.symbol,
        };
        this.tokenB = {
            address: data.token_b.address,
            decimals: data.token_b.decimals,
            isStablecoin: data.token_b.is_stablecoin,
            name: data.token_b.name,
            symbol: data.token_b.symbol,
        };
        this.updatedAt = new Date(data.updated_at);
        this.yieldTokenA = data.yield_token_a;
        this.yieldTokenAUsd = data.yield_token_a_usd;
        this.yieldTokenB = data.yield_token_b;
        this.yieldTokenBUsd = data.yield_token_b_usd;
        this.yieldTotalUsd = data.yield_total_usd;
    }
  
    get tickerPrice() {
        return ticker.prices[this.tokenA.symbol] && this.tokenB.isStablecoin ? ticker.prices[this.tokenA.symbol] : this.currentPrice;
    }

    get tickerInRange() {
        return this.tickerPrice >= this.rangeLower && this.tickerPrice <= this.rangeUpper;
    }

    get estimated24hYield() {
        const timeElapsedMs = this.updatedAt.getTime() - this.createdAt.getTime();
        const timeElapsedHours = timeElapsedMs / (1000 * 60 * 60);
    
        // If yield or elapsed time is 0 or negative, return 0
        if (this.yieldTotalUsd <= 0 || timeElapsedHours <= 0) {
            return 0;
        }
    
        // Calculate hourly yield percentage
        const hourlyReturnPercentage = (this.yieldTotalUsd / this.balanceTotalUsd) * 100 / timeElapsedHours;
    
        // Scale it to a 24-hour period
        return hourlyReturnPercentage * 24;
    }

    get estimated24hYieldUsd() {
        return this.balanceTotalUsd * this.estimated24hYield / 100;
    }
}
  