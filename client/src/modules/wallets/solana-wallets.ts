import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import api from '../server/api';
import { poolManager } from '..';
// import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';

export const connection = new Connection(clusterApiUrl('mainnet-beta'));

export class SolanaWalletManager {
    publicKeys: {
        key: PublicKey,
        type: string
    }[] = [];

    get browserWalletKey() {
        return this.publicKeys.find(k => k.type === 'browser')?.key;
    }

    async connect() {
        const provider = window['solana'];
        if (!provider || !provider.isPhantom) {
            console.error('Phantom Wallet not found. Please install it.');
            return;
        }

        try {
            const response = await provider.connect();
            const publicKey = new PublicKey(response.publicKey.toString());
            console.log('Connected to Phantom Wallet:', publicKey.toString());
            if (!this.publicKeys.find(k => k.key.equals(publicKey))) {
                this.publicKeys.push({
                    key: publicKey,
                    type: 'browser'
                });
            }
            const walletPositions = await api.poolManager.connectBrowserWallet(publicKey.toString());
            console.log('Wallet positions:', walletPositions);
            poolManager.updateManagedPositions(walletPositions);
            console.log('Connected to server with wallet:', publicKey.toString());
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
            const keyToDisconnect = this.publicKeys.find(k => k.type === 'browser');
            if (keyToDisconnect) {
                this.publicKeys.splice(this.publicKeys.indexOf(keyToDisconnect), 1);
            }
            const removedPositions = await api.poolManager.disconnectBrowserWallet();
            poolManager.removeManagedPositions(removedPositions);
            console.log('Disconnected from Phantom Wallet:', keyToDisconnect?.key.toString());


        } catch (error) {
            console.error('Disconnection failed:', error);
        }
    }
    
    async signMessage(message: string): Promise<{ signature: Uint8Array; publicKey: string } | null> {
        const provider = window['solana'];
        if (!provider || !provider.isPhantom) {
            console.error('Phantom Wallet not found. Please install it.');
            return null;
        }

        try {
            const encodedMessage = new TextEncoder().encode(message);
            const signedMessage = await provider.signMessage(encodedMessage, 'utf8');
            console.log('Message signed:', signedMessage);
            return {
                signature: signedMessage.signature,
                publicKey: signedMessage.publicKey.toString()
            };
        } catch (error) {
            console.error('Signing failed:', error);
            return null;
        }
    }
}
