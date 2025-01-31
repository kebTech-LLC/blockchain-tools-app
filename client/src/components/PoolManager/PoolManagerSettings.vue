<template>
    <div id="pool-manager-settings">
        <div class="position-settings">
            <div class="position-setting new">
                <input type="text" placeholder="Name" v-model="newPositionSettings.name">
                <input type="number" placeholder="Range Factor" v-model="newPositionSettings.rangeFactor">%
                <button @click="addNewPositionSettings">Add</button>
            </div>
            <div v-for="positionSetting in poolManager.positionSettings" :key="positionSetting.name">
                <div class="position-setting">
                    <div class="name">{{ positionSetting.name }}</div>
                    <input type="number" v-model="positionSetting.rangeFactor">%
                    <button @click="positionSetting.update()">Update</button>
                    <button @click="positionSetting.delete()">Delete</button>
                </div>    
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { poolManager } from '@/modules';
import { PositionSettings } from '@/modules/pool-manager/position-settings';
import { defineComponent, ref } from 'vue'

export default defineComponent({
    setup () {
        const newPositionSettings = ref(new PositionSettings({
            name: '',
            rangeFactor: 0
        }))

        const addNewPositionSettings = () => {
            newPositionSettings.value.create();
            newPositionSettings.value = new PositionSettings({
                name: '',
                rangeFactor: 0
            });
        }

        return {
            poolManager,
            newPositionSettings,
            addNewPositionSettings
        }
    }
})
</script>

<style scoped>
#pool-manager-settings {
    display: flex;
    width: 100%;
    gap: 1rem;
}
</style>