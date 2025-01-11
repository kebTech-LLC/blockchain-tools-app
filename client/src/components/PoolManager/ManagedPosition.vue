<template>
    <div class="managed-position">
        <div class="pool-type">{{ position.poolType }}</div>
        <div>{{ position.walletKey }}</div>
        <div class="name">{{ position.tokenA.symbol + '/' +  position.tokenB.symbol }}</div>
        <div class="current-price">{{ position.tickerPrice }}</div>
        <div class="range">
            <div class="lower">{{ position.rangeLower }}</div>
            <div class="upper">{{ position.rangeUpper }}</div>
        </div>
        <div class="balance">
            <div class="token-a">{{ position.balanceTokenAUsd.toFixed(2) }}</div>
            <div class="token-b">{{ position.balanceTokenBUsd.toFixed(2) }}</div>
            <div class="total">{{ position.balanceTotalUsd.toFixed(2) }}</div>
        </div>
        <div class="yield">
            <div class="yield-token-a" :title="position.yieldTokenAUsd.toString()">{{ position.yieldTokenAUsd.toFixed(2) }}</div>
            <div class="yield-token-b" :title="position.yieldTokenBUsd.toString()">{{ position.yieldTokenBUsd.toFixed(2) }}</div>
            <div class="yield-total" :title="position.yieldTotalUsd.toString()">{{ position.yieldTotalUsd.toFixed(2) }}</div>
            <div class="yield-percent" :title="position.estimated24hYield.toString()">{{ `${position.estimated24hYield.toFixed(2)}% ($${position.estimated24hYieldUsd.toFixed(2)})` }}</div>
        </div>
        <font-awesome-icon class="ellipsis" :icon="['fa', 'ellipsis']" @click.stop="FloatingMenu.open($event, FloatingMenuType.ManagedPosition, position)" />
    </div>
</template>

<script lang="ts">
import { ticker } from '@/modules';
import { ManagedPosition } from '@/modules/pool-manager/managed-position';
import { FloatingMenu, FloatingMenuType } from '@/state/views/floating-menu';
import { defineComponent } from 'vue'

export default defineComponent({
    props: {
        position: {
            type: ManagedPosition,
            required: true
        }
    },
    setup (props) {

        return {
            props,
            ticker,
            FloatingMenu,
            FloatingMenuType
        }
    }
})
</script>

<style scoped>
.managed-position {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
    border: 1px solid #ccc;
    border-radius: 0.5rem;
    text-align: center;
    min-width: 200px;
}
.pool-type {
    font-weight: bold;
    font-size: 1.2rem;
}
.range {
    color: v-bind("props.position.tickerInRange ? 'green' : 'red'");
}
</style>