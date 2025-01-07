import { reactive } from "vue";
import { Ticker } from "./ticker";
import { Server } from "./server";
import { Launcher } from "./launcher";

export const ticker = reactive(new Ticker());
export const server = reactive(new Server());
export const launcher = reactive(new Launcher());
