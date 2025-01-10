import { reactive } from "vue";
import { SolanaWalletManager } from "./solana-wallets";


export const solanaWalletManager = reactive(new SolanaWalletManager());