<template>
    <div id="floating-menu">
        <div v-for="item in views.floatingMenu.items" class="item" @click="item.action">
            <font-awesome-icon v-if="item.icon" class="icon" :icon="['fa', item.icon]" />
            <div class="label">{{ item.label }}</div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { views } from '@/state';

export default defineComponent({
    setup () {
        

        return {
            views
        }
    }
})
</script>

<style scoped>
.item {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    cursor: pointer;
    transition: background-color 0.2s;
}
</style>