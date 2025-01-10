<template>
	<Ticker />
	<WalletManager />
	<PoolManager />
</template>

<script lang="ts">
import PoolManager from './components/PoolManager/PoolManager.vue';
import Ticker from './components/Ticker.vue';
import WalletManager from './components/Wallets/WalletManager.vue';
import { launcher } from './modules';

export default {
	setup() {
		launcher.start();
		
		return {

		}
	},
	components: {
		Ticker,
		PoolManager,
		WalletManager
	}
}
</script>

<style scoped>

</style>
