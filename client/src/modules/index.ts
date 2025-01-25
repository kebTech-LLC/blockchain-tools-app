import { reactive } from "vue";
import { Ticker } from "./ticker";
import { Server } from "./server";
import { Launcher } from "./launcher";
import { PoolManager } from "./pool-manager";
import { Solana } from "./solana";
import { Auth } from "./auth";

export const ticker = reactive(new Ticker());
export const server = reactive(new Server());
export const launcher = reactive(new Launcher());
export const poolManager = reactive(new PoolManager());
export const solana = reactive(new Solana())
export const auth = reactive(new Auth());

window['modules'] = { ticker, server, launcher, poolManager, solana, auth };