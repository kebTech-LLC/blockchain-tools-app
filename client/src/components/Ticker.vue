<template>
    <div class="ticker">
        <div class="prices">
            <div class="price">
                <span class="label">SOL</span>
                <span class="value">{{ ticker.prices.SOL }}</span>
            </div>
            <div class="price">
                <span class="label">BTC</span>
                <span class="value">{{ ticker.prices.BTC }}</span>
            </div>
            <div class="price">
                <span class="label">ETH</span>
                <span class="value">{{ ticker.prices.ETH }}</span>
            </div>  
        </div>
        <button @click="showStats = !showStats">Toggle Stats</button>
        <div class="stats" v-show="showStats">
            <div v-for="stat in ticker.stats" class="stat">{{ stat }}</div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { ticker } from '../modules';

export default defineComponent({
    setup () {
        const showStats = ref(false);

        return {
            ticker,
            showStats
        }
    }
})
</script>

<style scoped>
.ticker {
    width: 100%;
}
.prices {
    display: flex;
    justify-content: space-around;
    align-items: center;
    width: 100%;
    font-size: 1.5rem;
    font-weight: bold;
}
.price {
    width: 100%;
    text-align: center;
}
.label {
    margin-right: 0.5rem;
}
.stats {
    display: flex;
    flex-direction: column;
}
</style>