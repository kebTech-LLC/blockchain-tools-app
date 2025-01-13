<template>
    <div class="new-position">
        <div class="top">
            <font-awesome-icon class="reset" :icon="['fa', 'rotate-left']" @click.stop="position.reset()" />
            <div class="pool-name">{{ position.pool.name }}</div>
            <font-awesome-icon class="close" :icon="['fa', 'times']" @click.stop="poolManager.closeNewPosition()" />
        </div>
        <div class="price">${{ position.pool.tickerPrice }}</div>
        <div class="bottom">
            <div class="position">
                <div class="range-heading">Range</div>    
                <div class="range-mode-selector">
                    <button @click="position.setDynamicRange()" :class="{ 'selected': position.dynamicRange }">Dynamic</button>
                    <button @click="position.setDynamicRange(false)" :class="{ 'selected': !position.dynamicRange }">Fixed</button>
                </div>
                <div v-if="!position.dynamicRange" class="">{{ (position.percentage * 100).toFixed(2) }}%</div>
                <div v-if="position.dynamicRange" class="percentages">
                    <div class="percentage">
                        <input ref="customPercentage" 
                            class="custom" 
                            step="0.1" 
                            type="number" 
                            value=".1" 
                            min="0.1" 
                            max="100" 
                            @input="position.adjustPercentage(parseFloat(($event.target as HTMLInputElement).value))"
                        />
                    </div>
                    
                    <div v-for="percentage in percentageOptions" 
                        class="percentage"
                        :class="{ 'selected': position.percentage === percentage / 100}"
                        @click="position.adjustPercentage(percentage)">
                        {{ percentage }}%
                    </div>
                </div>
                <div class="range">
                    <div v-if="position.dynamicRange">{{ position.rangeLower.toFixed(4) }}</div>
                    <div v-if="position.dynamicRange">{{ position.rangeUpper.toFixed(4) }}</div>
                    <input v-if="!position.dynamicRange" 
                        type="number" 
                        v-model="position.manualRangeLower" 
                        :max="position.manualRangeUpper * .99999999999"
                    />
                    <input v-if="!position.dynamicRange" 
                        type="number" 
                        v-model="position.manualRangeUpper" 
                        :min="position.manualRangeLower * 1.0000000001" 
                    />
                </div>
                <div class="distribution">
                    <input class="slider" type="range" min="0" max="100" v-model="position.distribution" step="1" />
                </div>
                <div>Total Amount</div>
                <input type="number" v-model="position.amountTotal" :max="position.walletBalanceTotal" />
                <div class="token-percentages">
                    <div class="token">
                        <div class="token-symbol">{{ position.pool.tokenA.symbol }}</div>
                        <div class="token-percentage">{{ position.distributionA.toFixed(2) }}%</div>
                        <div class="token-amount">${{ position.amountA.toFixed(2) }}</div>
                    </div>
                    <div class="token">
                        <div class="token-symbol">{{ position.pool.tokenB.symbol }}</div>
                        <div class="token-percentage">{{ position.distributionB.toFixed(2) }}%</div>
                        <div class="token-amount">${{ position.amountB.toFixed(2) }}</div>
                    </div>
                </div>
                
            </div>
            <div class="wallet-info">
                <div class="heading">Wallet</div>
                <select id="wallet" v-model="position.wallet">
                    <option v-for="wallet in solana.wallets" 
                        :key="wallet.pubkey.toString()" 
                        :value="wallet">
                        {{ wallet.name }}
                    </option>
                </select>
                <div class="balances">
                    <div class="balance">
                        <div class="token-symbol">{{ position.pool.tokenA.symbol }} Balance</div>
                        <div class="token-amount">${{ position.walletBalanceTokenA.toFixed(2) }}</div>
                    </div>
                    <div class="balance">
                        <div class="token-symbol">{{ position.pool.tokenB.symbol }} Balance</div>
                        <div class="token-amount">${{ position.walletBalanceTokenB.toFixed(2) }}</div>
                    </div>
                    <div class="balance">
                        <div class="token-symbol">Total Balance</div>
                        <div class="token-amount">${{ (position.walletBalanceTokenA + position.walletBalanceTokenB).toFixed(2) }}</div>
                    </div>
                </div>
            </div>
        </div>
        
        
        
       
        <button class="open-button" @click="poolManager.openPosition(position)">Open Position</button>
    </div>
</template>

<script lang="ts">
import { poolManager, solana } from '@/modules';
import { computed, defineComponent, ref, watchEffect } from 'vue';

export default defineComponent({
    setup () {
        const customPercentage = ref(null as HTMLInputElement | null);
        const position = poolManager.newPosition!; 
        const percentageOptions = [0.25, 0.5, 1, 2.5, 5];

        watchEffect(() => {
            position.recalculate();
        });

        watchEffect(() => {
            position.calculateWalletBalance();
            
        })

        return {
            poolManager,
            position,
            solana,
            percentageOptions,
            customPercentage
        }
    }
})
</script>

<style scoped>
.new-position {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 1rem;
    padding: 1rem;
    border-radius: 5px;
    border: 1px solid #ccc;
    padding-top: .5rem;
    margin-left: 1rem;
    margin-right: 1rem;
    align-items: center;
}

.pool-name {
    font-size: 1.5rem;
    font-weight: bold;
    width: fit-content;
}

.price {
    font-size: 1.5rem;
    text-align: center;
}
.top {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
}
.reset {
    cursor: pointer;
    font-size: 1.5rem;
}
.close {
    cursor: pointer;
    font-size: 1.5rem;
}
.bottom {
    display: flex;
    gap: 1rem;
}
.position {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    align-items: center;
    position: relative;
    border-radius: 5px;
    border: 1px solid #cccccc50;

}

.wallet-info {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    align-items: center;
    position: relative;
    border-radius: 5px;
    border: 1px solid #cccccc50;
    text-align: center;
}

@media (prefers-color-scheme: dark) {
    .new-position {
        color: rgba(255, 255, 255, 0.87);
        background-color: #242424;
    }
}
@media (prefers-color-scheme: light) {
    .new-position {
        color: #213547;
        background-color: #ffffff;
    }
}


.range-mode-selector {
    display: flex;
    gap: 1rem;
}

.range {
    display: flex;
    gap: 1rem;
}
.distribution {
    width: 100%;
}
.slider {
    width: 100%;
}
.percentages {
    display: flex;
    gap: .5rem;
}

.percentage {
    cursor: pointer;
    border: 1px solid #ccc;
    padding: .25rem;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.custom {
    width: 40px;
    height: 40px;
    border: none;
    background: none;
    text-align: center;
    justify-content: center;
    display: flex;
    border-radius: 50%;
}

.token-percentages {
    display: flex;
    gap: 1rem;
    text-align: center;
}

.token {
    display: flex;
    flex-direction: column;
}


.balances {
    display: flex;
    gap: 1rem;
    flex-direction: column;
    align-items: center;
}
.selected {
    outline: 2px solid #ccc;
}
.open-button {
    width: fit-content;
    margin-left: auto;
    margin-right: auto;
}
</style>