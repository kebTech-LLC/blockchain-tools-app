import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';

export const connection = new Connection(clusterApiUrl('mainnet-beta'));

export class SolanaWalletManager {
    publicKey: PublicKey | null = null;
    private adapter: PhantomWalletAdapter;

    constructor() {
        this.adapter = new PhantomWalletAdapter();
    }

    async connect() {
        try {
            await this.adapter.connect();
            this.publicKey = this.adapter.publicKey;
            console.log('Connected to:', this.publicKey?.toBase58());
        } catch (error) {
            console.error('Connection failed:', error);
        }
    }

    async disconnect() {
        try {
            this.adapter.disconnect();
            this.publicKey = null;
            console.log('Disconnected.');
        } catch (error) {
            console.error('Disconnection failed:', error);
        }
    }
}
