import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
// import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';

export const connection = new Connection(clusterApiUrl('mainnet-beta'));

export class SolanaWalletManager {
    publicKey: PublicKey | null = null;

    async connect() {
        const provider = window['solana'];
        if (!provider || !provider.isPhantom) {
            console.error('Phantom Wallet not found. Please install it.');
            return;
        }

        try {
            const response = await provider.connect();
            this.publicKey = new PublicKey(response.publicKey.toString());
            console.log('Connected to:', this.publicKey.toBase58());
        } catch (error) {
            console.error('Connection failed:', error);
        }
    }

    async disconnect() {
        const provider = window['solana'];
        if (!provider || !provider.isPhantom) {
            console.error('Phantom Wallet not found. Please install it.');
            return;
        }

        try {
            await provider.disconnect();
            this.publicKey = null;
            console.log('Disconnected.');
        } catch (error) {
            console.error('Disconnection failed:', error);
        }
    }
}
