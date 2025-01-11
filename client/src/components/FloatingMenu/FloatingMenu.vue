<template>
    <div id="floating-menu"
        ref="floatingMenu"
        :style="menuStyles">
        <div v-for="option in views.floatingMenu?.options">
            <div v-if="option.lineType === 'divider'" class="divider"></div>
            <div v-if="option.lineType === 'content'" class="option" @click.stop.prevent="() => {
                    option.action();
                    FloatingMenu.close()
                }">
                <div class="name">{{ option.label }}</div>
                <img v-if="option.image" :src="option.image" class="image" />
            </div>
            <div v-if="option.lineType === 'select'" class="option option-select">
                    <select class="select" 
                        @change="($event) => {
                        option.data.selected = ($event.target as HTMLSelectElement).value;
                        console.log('selected', option.data.selected)
                        option.data.action();
                        FloatingMenu.close()
                    }">
                        <option disabled selected>{{ option.label }}</option>
                        <option v-for="item in option.data.options" :value="item.id">{{ item.name }}</option>
                    </select>
            </div>
            <div v-if="option.lineType === 'multi-select'" class="option" @click="option.action">
                <div class="name">{{ option.label }}</div>
                <div v-if="option.data.showSubMenu" class="sub-menu">
                    <div v-for="item in option.data.options" 
                        class="option suboption" 
                        @click.stop="() => {
                            if (option.data.selected.includes(item)) {
                                option.data.selected = option.data.selected.filter((newItem: any) => newItem !== item);
                            } else {
                                option.data.selected.push(item);
                            }
                            option.data.action()
                        }"
                        :class="{ 'selected': option.data.selected.includes(item) }">
                        <div class="name">{{ item.name }}</div>
                    </div>
                </div>
            </div>
            <div v-if="option.lineType === 'heading'" class="option">
                <div class="name heading">{{ option.label }}</div>
            </div>
            <div v-if="option.lineType === 'space'" class="space"></div>  
        </div>
    </div>
</template>

<script lang="ts">
import { computed, defineComponent, onMounted, ref } from 'vue';
import { views } from '@/state';
import { FloatingMenu } from '@/state/views/floating-menu';


export default defineComponent({
    setup () {
        // const width = ref(0);
        // const height = ref(0);
        const floatingMenu = ref(null as HTMLDivElement | null);

        const menuStyles = computed(() => {
            console.log('floatingMenu', floatingMenu.value)
            const width = floatingMenu.value?.offsetWidth || 0;
            const height = floatingMenu.value?.offsetHeight || 0;

            console.log('width', width)
            console.log('height', height)

            return {
                'left': views.floatingMenu?.position.includes('right') ? views.floatingMenu?.x - width - 10 + 'px' : views.floatingMenu?.x + 'px',
                'top': views.floatingMenu?.position.includes('bottom') ? views.floatingMenu?.y - height + 'px' : views.floatingMenu?.y + 'px',
                // 'bottom-right': views.floatingMenu?.position === 'bottom-right',
                // 'bottom-left': views.floatingMenu?.position === 'bottom-left',
            }
        });

        return {
            views,
            menuStyles,
            floatingMenu,
            FloatingMenu
        }
    }
})
</script>

<style scoped>
#floating-menu {
    position: absolute;
    backdrop-filter: blur(50px);
    color: white;
    padding: .5rem;
    border: 1px solid #ccc;
    border-radius: 5px;
    width: 200px;
    z-index: 101;
    text-align: left;
    font-size: .9rem;
}
.option {
    padding: .5rem;
    padding-bottom: .25rem;
    padding-top: .25rem;
    cursor: pointer;
    border-radius: 5px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: .5rem;
    font-size: 1rem;
    font-family: Avenir,Helvetica,Arial,sans-serif;

}
@media (prefers-color-scheme: dark) {
    #floating-menu {
        color: rgba(255, 255, 255, 0.87);
        background-color: #242424;
    }
    .option:hover {
        color: #213547;
        background-color: #ffffff;
    }
}
@media (prefers-color-scheme: light) {
    #floating-menu {
        color: #213547;
        background-color: #ffffff;
    }
    .option:hover {
        color: rgba(255, 255, 255, 0.87);
        background-color: #242424;
    }
}
img {
    width: 30px;
    height: 30px;
    border-radius: 2px;
}
.divider {
    border-top: 1px solid #ffffff50;
    margin-top: .5rem;
    margin-bottom: .5rem;
}
select:focus-visible {
    outline: none;
    border: none;
}
select {
    width: 100%;
    padding: .25rem;
    border-radius: 5px;
    border: none;
    background: none;
    color: white;
    outline: none;
    font-size: 1rem;
    font-family: Avenir,Helvetica,Arial,sans-serif;
}
.option-select {
    padding: 0;
}
.selected {
    background: #ffffff20;
}
.suboption {
    margin-top: .25rem;
}
.heading {
    font-size: 1.2rem;
    font-weight: bold;
}
.space {
    height: 1rem;
}
</style>