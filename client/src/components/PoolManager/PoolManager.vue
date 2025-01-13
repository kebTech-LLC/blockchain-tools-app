<template>
    <div id="pool-manager">
        <NewPosition v-if="poolManager.newPosition" />
        <div class="positions">
            
            <ManagedPosition v-for="position in poolManager.managedPositions" :key="position.address" :position="position" />
        </div>
        <div class="pools">
            <OrcaPool v-for="pool in poolManager.orcaPools" :key="pool.address" :pool="pool" @new="" />
        </div>
        
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import ManagedPosition from './ManagedPosition.vue';
import { poolManager } from '@/modules';
import OrcaPool from './OrcaPool.vue';
import NewPosition from './NewPosition.vue';

export default defineComponent({
    setup () {


        return {
            poolManager
        }
    },
    components: {
        ManagedPosition,
        OrcaPool,
        NewPosition,
    }
})
</script>

<style scoped>
#pool-manager {
    display: flex;
    width: 100%;
    flex-direction: column;
    gap: 1rem;
}
.positions {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    position: relative;
    overflow: scroll;
    align-items: center;
    /* min-height: 430px; */
}
.pools {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    padding: .5rem;
}
</style>