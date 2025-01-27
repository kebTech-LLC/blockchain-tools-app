<template>
    <div class="orca-pool" ref="menuContainer">
        <div class="field name">{{ pool.name + ' ' + (pool.feeRate * .0001) + '%' }}</div>
        <div class="field yield">{{ (pool.yieldOverTvl * 100).toFixed(2) + '%' }}</div>
        <div class="field tvl">{{ '$' + new Intl.NumberFormat().format(Number(pool.tvlUsdc.toFixed(2))) }}</div>
        <div class="field 24volume">{{ '$' + new Intl.NumberFormat().format(Number(pool.volumeUsdc24h.toFixed(2))) }}</div>
        <div class="field fees">{{ '$' + new Intl.NumberFormat().format(Number(pool.feesUsdc24h.toFixed(2))) }}</div>
        <font-awesome-icon class="ellipsis" :icon="['fa', 'ellipsis']" @click.stop="FloatingMenu.open($event, FloatingMenuType.Pool, pool)" />
    </div>  
</template>

<script lang="ts">
import { OrcaPool } from '@/modules/pool-manager/orca-pool';
import { FloatingMenu, FloatingMenuType } from '@/state/views/floating-menu';
import { defineComponent, onMounted, onUnmounted, ref } from 'vue'

export default defineComponent({
    props: {
        pool: {
            type: OrcaPool,
            required: true
        }
    },
    setup (props) {

        return {
            props,
            FloatingMenu,
            FloatingMenuType
        }
    }
})
</script>

<style scoped>
.orca-pool {
    display: flex;
    padding: .5rem;
    border: 1px solid #ccc;
    border-radius: 5px;
    justify-content: space-between;
    position: relative;
}
.field {
    width: 100%;
}
</style>