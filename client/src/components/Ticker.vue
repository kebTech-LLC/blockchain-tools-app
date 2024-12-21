<template>
    <div class="ticker">
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
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { ticker } from '../modules';

export default defineComponent({
    setup () {
        

        return {
            ticker
        }
    }
})
</script>

<style scoped>
.ticker {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100%;
    font-size: 1.5rem;
    font-weight: bold;
}
.price {
    display: flex;
    gap: 0.5rem;
}
</style>