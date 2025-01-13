import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { Wallet } from "./wallet";
import api from "../server/api";
import { ticker } from "..";

const TOKEN_PROGRAM_KEY = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
const SOLANA_TOKEN_MINT = 'So11111111111111111111111111111111111111112';

export class Solana {
    client: Connection;
    wallets: Wallet[]

    constructor() {
        this.client = new Connection(this.rpcUrls.alchemy, 'confirmed');
        this.wallets = [];
    }

    get rpcUrls() {
        const alchemyBase = 'https://solana-mainnet.g.alchemy.com/v2/';
        const alchemyApiKey = import.meta.env.VITE_ALCHEMY_API_KEY;

        const heliusBase = 'https://rpc.helius.xyz?api-key=';
        const heliusApiKey = import.meta.env.VITE_HELIUS_API_KEY;

        const quicknodeBase = 'https://fittest-bold-card.solana-mainnet.quiknode.pro/';
        const quicknodeApiKey = import.meta.env.VITE_QUICKNODE_API_KEY;

        return {
            alchemy: alchemyBase + alchemyApiKey,
            helius: heliusBase + heliusApiKey,
            quicknode: quicknodeBase + quicknodeApiKey,
            mainnet: 'https://api.mainnet-beta.solana.com',
        }
    }

    get localWallet() {
        return this.wallets.find(k => k.name === 'Local');
    }

    get programmaticWallet() {
        return this.wallets.find(k => k.name === 'Programmatic');
    }

    getWallet(keyString: string) {
        const pubkey = new PublicKey(keyString);
        return this.wallets.find(k => k.pubkey.equals(pubkey));
    }

    async populateProgrammaticWalletPubkey() {
        const walletPubkey = await api.poolManager.get.programmaticWalletPubkey();
        console.log('Programmatic wallet pubkey:', walletPubkey);
        if (!this.wallets.find(w => w.pubkey.equals(new PublicKey(walletPubkey)))) {
            this.wallets.push(new Wallet(new PublicKey(walletPubkey), 'Programmatic'));
        }
    }

    async populateLocalWalletPubkey() {
        const walletPubkey = await api.poolManager.get.storedLocalWalletPubkey();
        console.log('Local wallet pubkey:', walletPubkey);

        if (!this.wallets.find(w => w.pubkey.equals(new PublicKey(walletPubkey)))) {
            this.wallets.push(new Wallet(new PublicKey(walletPubkey), 'Local'));
        }
    }

    async getTokenBalance(walletPubkey: PublicKey, tokenMint: string): Promise<number> {
        if (tokenMint === SOLANA_TOKEN_MINT) {
            const balance = await this.client.getBalance(walletPubkey);
            const sol = balance / LAMPORTS_PER_SOL;
            return sol * ticker.prices.SOL;
        } else {
            const tokenAccount = await this.client.getParsedTokenAccountsByOwner(walletPubkey, { 
                programId: new PublicKey(TOKEN_PROGRAM_KEY),
                mint: new PublicKey(tokenMint)
            });
            return tokenAccount.value[0]?.account.data.parsed.info.tokenAmount.uiAmount || 0;
        }
    }
}