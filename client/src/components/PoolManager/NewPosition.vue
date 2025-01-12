<template>
    <div class="new-position">
        <div class="top">
            <font-awesome-icon class="reset" :icon="['fa', 'rotate-left']" @click="poolManager.setupNewPosition(position.pool)" />
            <font-awesome-icon class="close" :icon="['fa', 'times']" @click="poolManager.closeNewPosition()" />
        </div>
        
        <div class="pool-name">{{ position.pool.name }}</div>
        <div class="price">{{ position.pool.tickerPrice }}</div>
        <div class="percentages">
            <div class="percentage" @click="position.adjustPercentage(parseInt(customPercentage!.value))">
                <input ref="customPercentage" 
                    class="custom" 
                    step="0.1" 
                    type="number" 
                    value=".1" 
                    min="0.1" 
                    max="100" 
                    @change="position.adjustPercentage(parseInt(customPercentage!.value))"
                />
            </div>
            
            <div v-for="percentage in percentageOptions" 
                class="percentage"
                :class="{ 'selected': position.percentage === percentage / 100}"
                @click="position.adjustPercentage(percentage)">
                {{ percentage }}
            </div>
        </div>
        <div class="distribution">
            <input type="range" min="0" max="100" v-model="position.distribution" step="1" />
        </div>
        <div class="token-percentages">
            <div class="token">
                <div class="token-symbol">{{ position.pool.tokenA.symbol }}</div>
                <div class="token-percentage">{{ position.distributionA }}%</div>
            </div>
            <div class="token">
                <div class="token-symbol">{{ position.pool.tokenB.symbol }}</div>
                <div class="token-percentage">{{ position.distributionB }}%</div>
            </div>
            
        </div>
        <div class="range-mode-selector">
            <button @click="position.setDynamicRange()" :class="{ 'selected': position.dynamicRange }">Dynamic</button>
            <button @click="position.setDynamicRange(false)" :class="{ 'selected': !position.dynamicRange }">Fixed</button>
        </div>
        <div class="range">
            <div v-if="position.dynamicRange">{{ position.rangeLower }}</div>
            <div v-if="position.dynamicRange">{{ position.rangeUpper }}</div>
            <input v-if="!position.dynamicRange" type="number" v-model="position.manualRangeLower" />
            <input v-if="!position.dynamicRange" type="number" v-model="position.manualRangeUpper" />
        </div>
        <select id="wallet" v-model="position.walletKey">
            <option v-for="wallet in wallets.solanaWalletManager.publicKeys" 
                :key="wallet.key.toString()" 
                :value="wallet.key.toString()">
                {{ wallet.type }}
            </option>
        </select>
        <button @click="poolManager.openPosition(position)">Add Position</button>
    </div>
</template>

<script lang="ts">
import { poolManager, wallets } from '@/modules';
import { computed, defineComponent, ref, watchEffect } from 'vue';

export default defineComponent({
    setup () {
        const customPercentage = ref(null as HTMLInputElement | null);
        const position = computed(() => poolManager.newPosition!); 
        const percentageOptions = [0.1, 0.25, 0.5, 1, 2, 5, 10];

        watchEffect(() => {
            position.value.recalculate();
        });

        return {
            poolManager,
            position,
            wallets,
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
    gap: 1rem;
    padding: .5rem;
    align-items: center;
    position: absolute;
    left: 0;
    top: 0;
    right: 0;
    bottom: 0;
    margin: 1rem;
    border-radius: 5px;
    border: 1px solid #ccc;
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
}

.token {
    display: flex;
    flex-direction: column;
    gap: .5rem;
}

.top {
    display: flex;
    justify-content: space-between;
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

.selected {
    outline: 2px solid #ccc;
}
</style>