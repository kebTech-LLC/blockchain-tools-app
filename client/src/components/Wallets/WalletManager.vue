<template>
    <div id="wallet-manager">
        <button @click="walletManager.connect">Connect Wallet</button>
        <button v-if="walletManager.publicKey" @click="walletManager.disconnect">Disconnect</button>
        <p v-if="walletManager.publicKey">Connected to: {{ walletManager.publicKey }}</p>
    </div>
</template>
  
<script lang="ts">
import { wallets } from '@/modules';
import { defineComponent } from 'vue';
  
export default defineComponent({
    setup() {
        const walletManager = wallets.solanaWalletManager;
    
        return {
            walletManager
        }
    },
});
</script>
  