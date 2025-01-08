<template>
	<Ticker />
	<LiquidityPools />
</template>

<script lang="ts">
import LiquidityPools from './components/LiquidityPools/LiquidityPools.vue';
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
		LiquidityPools
	}
}
</script>

<style scoped>

</style>
