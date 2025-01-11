<template>
    <div id="wallet-manager">
        <button v-if="!browserKey" @click="walletManager.connect">Connect Wallet</button>
        <p v-if="browserKey">Connected to: {{ browserKey.key }}</p>
        <button v-if="browserKey" @click="walletManager.disconnect">Disconnect</button>
    </div>
</template>
  
<script lang="ts">
import { wallets } from '@/modules';
import api from '@/modules/server/api';
import { computed, defineComponent, watchEffect } from 'vue';
  
export default defineComponent({
    setup() {
        const walletManager = wallets.solanaWalletManager;

        const browserKey = computed(() => walletManager.publicKeys.find((key) => key.type === 'browser'));
        
        // watchEffect(async () => {
        //     console.log('browserKey', browserKey.value?.key)
        //     await api.poolManager.connectBrowserWallet(browserKey.value?.key || '');
        // })

        return {
            walletManager,
            browserKey
        }
    },
});
</script>
  
<style scoped>
#wallet-manager {
    display: flex;
    gap: 1rem;
    padding: 1rem;
}
</style>