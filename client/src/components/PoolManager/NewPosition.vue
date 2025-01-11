<template>
    <div class="new-position">
        <font-awesome-icon class="close" :icon="['fa', 'times']" @click="poolManager.closeNewPosition()" />
        <div class="pool-name">{{ position.pool.name }}</div>
        <div class="price">{{ position.pool.tickerPrice }}</div>
        <div class="range">
            <input type="number" v-model="position.rangeLower" />
            <input type="number" v-model="position.rangeUpper" />
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
import { computed, defineComponent } from 'vue';

export default defineComponent({
    setup () {
        const position = computed(() => poolManager.newPosition!); 

        return {
            poolManager,
            position,
            wallets
        }
    }
})
</script>

<style scoped>
.new-position {
    display: flex;
    flex-direction: column;
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
.close {
    align-self: flex-end;
    cursor: pointer;
}
</style>