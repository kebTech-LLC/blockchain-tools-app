import { Connection, LAMPORTS_PER_SOL, PublicKey, clusterApiUrl } from '@solana/web3.js';
import api from '../server/api';
import { poolManager, solana } from '..';



export class Wallet {
    pubkey: PublicKey;
    name: string;

    constructor(key: PublicKey, name: string) {
        this.pubkey = key;
        this.name = name;
    }

    async getTokenBalance(tokenMint: string): Promise<number> {
        return await solana.getTokenBalance(this.pubkey, tokenMint);
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
            if (solana.localWallet?.pubkey !== publicKey) {
                solana.wallets.push(new Wallet(publicKey, 'Local'));
                const walletPositions = await api.poolManager.connectLocalWallet(publicKey.toString());
                console.log('Wallet positions:', walletPositions);
                poolManager.updateManagedPositions(walletPositions);
                console.log('Connected to server with wallet:', publicKey.toString());
            }
    
           
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
            solana.wallets.splice(solana.wallets.indexOf(solana.localWallet!), 1);
            const removedPositions = await api.poolManager.disconnectLocalWallet();
            poolManager.removeManagedPositions(removedPositions);
            console.log('Disconnected server from local wallet');

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

