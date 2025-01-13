<template>
    <div id="wallet-manager">
        <!-- <button v-if="!browserKey" @click="walletManager.connect">Connect Wallet</button>
        <p v-if="browserKey">Connected to: {{ browserKey.key }}</p>
        <button v-if="browserKey" @click="walletManager.disconnect">Disconnect</button> -->
        <div class="wallets">
            <div v-for="wallet in solana.wallets" :key="wallet.pubkey.toString()">
                <div>{{ wallet.name }}</div>
                <div>{{ wallet.pubkey.toString() }}</div>
            </div>
        </div>
    </div>
</template>
  
<script lang="ts">
import { solana } from '@/modules';
import api from '@/modules/server/api';
import { computed, defineComponent, watchEffect } from 'vue';
  
export default defineComponent({
    setup() {

        const localWallet = computed(() => solana.localWallet);
        
        // watchEffect(async () => {
        //     console.log('browserKey', browserKey.value?.key)
        //     await api.poolManager.connectBrowserWallet(browserKey.value?.key || '');
        // })

        return {
            solana
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