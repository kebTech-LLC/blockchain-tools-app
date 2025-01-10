import { reactive } from "vue";
import { Ticker } from "./ticker";
import { Server } from "./server";
import { Launcher } from "./launcher";
import { PoolManager } from "./pool-manager";
import { solanaWalletManager } from "./wallets";

export const ticker = reactive(new Ticker());
export const server = reactive(new Server());
export const launcher = reactive(new Launcher());
export const poolManager = reactive(new PoolManager());
export const wallets = {
    solanaWalletManager
}

window['poolManager'] = poolManager;
window['wallets'] = wallets;