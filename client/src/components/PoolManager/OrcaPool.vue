<template>
    <div class="orca-pool">
        <div class="name">{{ (pool.tokenA.symbol + '/' + pool.tokenB.symbol) || '' }}</div>
        <div class="yield">{{ pool.yieldOverTvl }}</div>
        <div class="tvl">{{ pool.tvlUsdc }}</div>
        <div class="24volume">{{ pool.volumeUsdc24h }}</div>
        <div class="fees">{{ pool.feesUsdc24h }}</div>
    </div>  
</template>

<script lang="ts">
import { OrcaPool } from '@/modules/pool-manager/orca-pool';
import { defineComponent } from 'vue'

export default defineComponent({
    props: {
        pool: {
            type: OrcaPool,
            required: true
        }
    },
    setup (props) {
        

        return {
            props
        }
    }
})
</script>

<style scoped>
.orca-pool {
    display: flex;
    justify-content: space-evenly;
    padding: .5rem;
    border: 1px solid black;
    border-radius: 5px;
}
</style>