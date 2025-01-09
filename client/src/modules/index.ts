import { reactive } from "vue";
import { Ticker } from "./ticker";
import { Server } from "./server";
import { Launcher } from "./launcher";
import { PoolManager } from "./pool-manager";

export const ticker = reactive(new Ticker());
export const server = reactive(new Server());
export const launcher = reactive(new Launcher());
export const poolManager = reactive(new PoolManager());

window['poolManager'] = poolManager;