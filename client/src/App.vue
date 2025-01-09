<template>
	<Ticker />
	<PoolManager />
</template>

<script lang="ts">
import PoolManager from './components/PoolManager/PoolManager.vue';
import Ticker from './components/Ticker.vue';
import { launcher } from './modules';

export default {
	setup() {
		launcher.start();
		
		return {

		}
	},
	components: {
		Ticker,
		PoolManager
	}
}
</script>

<style scoped>

</style>
