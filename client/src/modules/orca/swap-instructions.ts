import { Keypair, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Buffer } from 'buffer';

type SwapQuote = ExactInSwapQuote | ExactOutSwapQuote;

type ExactInSwapQuote = {
    tokenIn: number;
    tokenEstOut: number;
    tokenMinOut: number;
    tradeFee: number;
} 

type ExactOutSwapQuote = {
    tokenOut: number;
    tokenEstIn: number;
    tokenMaxIn: number;
    tradeFee: number;
}

export class SwapInstructions {
    instructions: TransactionInstruction[];
    quote: SwapQuote;
    additionalSigners: Keypair[];

    constructor(data: any) {
        this.instructions = data.instructions.map((instr: any) => {
            const keys = instr.accounts.map((account: any) => ({
                pubkey: new PublicKey(account.pubkey),
                isSigner: account.is_signer,
                isWritable: account.is_writable,
            }));
            return new TransactionInstruction({
                keys,
                programId: new PublicKey(instr.program_id),
                data: Buffer.from(instr.data), // Explicitly use Buffer.from here
            });
        });
        this.additionalSigners = data.additional_signers.map((key: string) => {
            const secretKey = Uint8Array.from(atob(key), c => c.charCodeAt(0));
            return Keypair.fromSecretKey(secretKey);
        });
        this.quote = data.quote;
    }
}