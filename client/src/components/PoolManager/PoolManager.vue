<template>
    <div id="pool-managers">
        <div class="positions">
            <Position v-for="position in poolManager.managedPositions" :key="position.address" :position="position" />
        </div>
        <div class="pools">
            <OrcaPool v-for="pool in poolManager.orcaPools" :key="pool.address" :pool="pool" />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import Position from './Position.vue';
import { poolManager } from '@/modules';
import OrcaPool from './OrcaPool.vue';

export default defineComponent({
    setup () {
        

        return {
            poolManager
        }
    },
    components: {
        Position,
        OrcaPool
    }
})
</script>

<style scoped>
#pool-managers {
    display: flex;
    width: 100%;
    flex-direction: column;
    gap: 1rem;
}
.positions {
    display: flex;
    gap: 1rem;
    width: 100%;
    padding: 1rem;
}
.pools {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    padding: .5rem;
}
</style>