<template>
	<Ticker />
</template>

<script lang="ts">
import Ticker from './components/Ticker.vue';
import { launcher } from './modules';

export default {
	setup() {
		launcher.start();
		
		return {

		}
	},
	components: {
		Ticker
	}
}
</script>

<style scoped>

</style>
