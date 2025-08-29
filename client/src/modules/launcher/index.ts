
import { poolManager, server, solana } from "@/modules";
// import * as state from "@/state";
import { reactive, watchEffect } from "vue";

export class Launcher {

    async start() {
        console.log('Launcher started');
        await solana.populateLocalWalletPubkey();
        await solana.populateProgrammaticWalletPubkey();
        await poolManager.populatePositionSettings();
        // await poolManager.populateOrcaPools();
        await server.registerSocket();
        console.log('socket registered from launcher');
      
    }

}
