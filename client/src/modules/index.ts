import { reactive } from "vue";
import { Ticker } from "./ticker";

const ticker = reactive(new Ticker());

export {
    ticker
};