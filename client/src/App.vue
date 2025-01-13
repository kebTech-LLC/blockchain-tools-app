<template>
	<Ticker />
	<!-- <WalletManager /> -->
	<PoolManager />
	<Transition name="menu">
		<FloatingMenu v-if="views.floatingMenu && views.floatingMenu.options.length > 0" />
	</Transition>
</template>

<script lang="ts">
import FloatingMenu from './components/FloatingMenu/FloatingMenu.vue';
import PoolManager from './components/PoolManager/PoolManager.vue';
import Ticker from './components/Ticker.vue';
import WalletManager from './components/Wallets/WalletManager.vue';
import { launcher } from './modules';
import { views } from './state';

export default {
	setup() {
		launcher.start();
		
		return {
			views
		}
	},
	components: {
		Ticker,
		PoolManager,
		WalletManager,
		FloatingMenu
	}
}
</script>

<style scoped>
.menu-enter-active, .menu-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.menu-enter-from, .menu-leave-to {
  opacity: 0;
  transform: scale(0.8);
}

.menu-enter-to, .menu-leave-from {
  opacity: 1;
  transform: scale(1);
}
</style>
