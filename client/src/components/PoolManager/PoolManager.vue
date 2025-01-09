<template>
    <div id="liquidity-pools">
        <div class="positions">
            <Position v-for="position in poolManager.managedPositions" :key="position.address" :position="position" />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import Position from './Position.vue';
import { poolManager } from '@/modules';

export default defineComponent({
    setup () {
        

        return {
            poolManager
        }
    },
    components: {
        Position
    }
})
</script>

<style scoped>
#liquidity-pools {
    display: flex;
    width: 100%;
}
.positions {
    display: flex;
    gap: 1rem;
    width: 100%;
    padding: 1rem;
}
</style>