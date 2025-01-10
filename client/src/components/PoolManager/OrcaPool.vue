<template>
    <div class="orca-pool" ref="menuContainer">
        <div class="field name">{{ (pool.tokenA.symbol + '/' + pool.tokenB.symbol) || '' }}</div>
        <div class="field yield">{{ (pool.yieldOverTvl * 100).toFixed(2) + '%' }}</div>
        <div class="field tvl">{{ '$' + new Intl.NumberFormat().format(Number(pool.tvlUsdc.toFixed(2))) }}</div>
        <div class="field 24volume">{{ '$' + new Intl.NumberFormat().format(Number(pool.volumeUsdc24h.toFixed(2))) }}</div>
        <div class="field fees">{{ '$' + new Intl.NumberFormat().format(Number(pool.feesUsdc24h.toFixed(2))) }}</div>
        <font-awesome-icon class="ellipsis" :icon="['fa', 'ellipsis']" @click="handleMenuClick" />
        <div v-show="showMenu" class="menu">
            <div class="menu-item" @click="openPosition">Open Position</div>
            <div class="menu-item">Add to Favorites</div>
        </div>
    </div>  
</template>

<script lang="ts">
import { OrcaPool } from '@/modules/pool-manager/orca-pool';
import { defineComponent, onMounted, onUnmounted, ref } from 'vue'

export default defineComponent({
    props: {
        pool: {
            type: OrcaPool,
            required: true
        }
    },
    setup (props, { emit }) {
        const showMenu = ref(false);
        const menuContainer = ref<HTMLElement | null>(null);
        const menuRight = ref(0);
        const menuTop = ref(0);

        const toggleMenu = () => {
            showMenu.value = !showMenu.value;
        };

        const handleMenuClick = (e: MouseEvent) => {
            e.stopPropagation();
            menuRight.value = window.innerWidth - e.clientX;
            menuTop.value = e.clientY;
            toggleMenu();
        };

        const handleClickOutside = (event: MouseEvent) => {
            if (menuContainer.value && !menuContainer.value.contains(event.target as Node)) {
                showMenu.value = false;
            }
        };

        const openPosition = () => {
            emit('new', props.pool);
        };

        onMounted(() => {
            document.addEventListener('click', handleClickOutside);
        });

        onUnmounted(() => {
            document.removeEventListener('click', handleClickOutside);
        });

        return {
            props,
            showMenu,
            menuContainer,
            toggleMenu,
            handleClickOutside,
            handleMenuClick,
            menuRight,
            menuTop,
            openPosition
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
}
.field {
    width: 100%;
}
.ellipsis {
    cursor: pointer;
    width: fit-content;
    margin-top: auto;
    margin-bottom: auto;
    font-size: 1.5rem;
}
.menu {
    position: absolute;
    border: 1px solid #ccc;
    border-radius: 5px;
    padding: .5rem;
    z-index: 1;
    right: v-bind("menuRight + 'px'");
    top: v-bind("menuTop + 'px'");
    background: #ccc;
}
@media (prefers-color-scheme: dark) {
    .menu {
        color: rgba(255, 255, 255, 0.87);
        background-color: #242424;
    }
    .menu-item:hover {
        color: #213547;
        background-color: #ffffff;
    }
}

@media (prefers-color-scheme: light) {
    .menu {
        color: #213547;
        background-color: #ffffff;
    }
    .menu-item:hover {
        color: rgba(255, 255, 255, 0.87);
        background-color: #242424;
    }
}
.menu-item {
    cursor: pointer;
    padding: .5rem;
    border-radius: 5px;
}

</style>