// import { Visitor } from "./visitor";
import { reactive } from "vue";
import { Views } from "./views";

export const views = reactive(new Views());
// export const visitor = reactive(new Visitor());

window['state'] = { views };