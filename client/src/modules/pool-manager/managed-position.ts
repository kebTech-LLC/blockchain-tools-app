import { poolManager, ticker } from "..";
import api from "../server/api";
import utils from "../utils";

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
    sqrtPrice: bigint;
    tickSpacing: number;
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
    walletKey: string;
    yieldTokenA: number;
    yieldTokenAUsd: number;
    yieldTokenB: number;
    yieldTokenBUsd: number;
    yieldTotalUsd: number;
    autoRebalance: boolean;

  
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
        this.sqrtPrice = BigInt(data.sqrt_price);
        this.tickSpacing = data.tick_spacing;
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
        this.walletKey = data.wallet_key;
        this.yieldTokenA = data.yield_token_a;
        this.yieldTokenAUsd = data.yield_token_a_usd;
        this.yieldTokenB = data.yield_token_b;
        this.yieldTokenBUsd = data.yield_token_b_usd;
        this.yieldTotalUsd = data.yield_total_usd;
        this.autoRebalance = data.auto_rebalance;
    }

    toSnakeCase() {
        return {
            address: this.address,
            balance_token_a: this.balanceTokenA,
            balance_token_a_percentage: this.balanceTokenAPercentage,
            balance_token_a_usd: this.balanceTokenAUsd,
            balance_token_b: this.balanceTokenB,
            balance_token_b_percentage: this.balanceTokenBPercentage,
            balance_token_b_usd: this.balanceTokenBUsd,
            balance_total_usd: this.balanceTotalUsd,
            created_at: this.createdAt,
            current_price: this.currentPrice,
            current_ticker_price: this.tickerPrice,
            in_range: this.inRange,
            pool_address: this.poolAddress,
            pool_type: this.poolType,
            position_mint: this.positionMint,
            range_lower: this.rangeLower,
            range_upper: this.rangeUpper,
            reward_infos: this.rewardInfos.map(info => ({
                amount_owed: info.amountOwed,
                growth_inside_checkpoint: info.growthInsideCheckpoint,
            })),
            rewards_owed: this.rewardsOwed,
            tick_spacing: this.tickSpacing,
            sqrt_price: this.sqrtPrice.toString(),
            token_a: {
                address: this.tokenA.address,
                decimals: this.tokenA.decimals,
                is_stablecoin: this.tokenA.isStablecoin,
                name: this.tokenA.name,
                symbol: this.tokenA.symbol,
            },
            token_b: {
                address: this.tokenB.address,
                decimals: this.tokenB.decimals,
                is_stablecoin: this.tokenB.isStablecoin,
                name: this.tokenB.name,
                symbol: this.tokenB.symbol,
            },
            updated_at: this.updatedAt,
            wallet_key: this.walletKey,
            yield_token_a: this.yieldTokenA,
            yield_token_a_usd: this.yieldTokenAUsd,
            yield_token_b: this.yieldTokenB,
            yield_token_b_usd: this.yieldTokenBUsd,
            yield_total_usd: this.yieldTotalUsd,
            auto_rebalance: this.autoRebalance,
        };
    }

    get timeCreated() {
        return utils.cleanDate(this.createdAt)
    }

    get durationActive() {  
        const now = new Date();
        const durationMs = now.getTime() - this.createdAt.getTime();
        const durationMinutes = Math.floor(durationMs / (1000 * 60));
        return durationMinutes + ' minutes';
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

    get rangeFactor() {
        const middle = (this.rangeUpper + this.rangeLower) / 2;
        return (this.rangeUpper - middle) / middle;
    }

    async close() {
        await poolManager.closePosition(this);
        console.log('closed position', this);
    }

    async toggleAutoRebalance() {
        await  api.poolManager.toggleAutoRebalance(this);
        console.log('toggled auto-rebalance', this);
    }
}
  