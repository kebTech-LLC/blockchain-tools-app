import { Keypair, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Buffer } from 'buffer';

// class CollectRewardQuote {
//     rewardsOwed: number;

//     constructor(data: any) {
//         this.rewardsOwed = data.rewards_owed;
//     }
// }

// class CollectRewardsQuote {
//     rewards: CollectRewardQuote[];

//     constructor(data: any) {
//         this.rewards = data.rewards.map((reward: any) => new CollectRewardQuote(reward));
//     }
// }

class CollectFeesQuote {
    feeOwedA: number;
    feeOwedB: number;

    constructor(data: any) {
        this.feeOwedA = data.fee_owed_a;
        this.feeOwedB = data.fee_owed_b;
    }
}

class DecreaseLiquidityQuote {
    liquidityDelta: number;
    tokenEstA: number;
    tokenEstB: number;
    tokenMinA: number;
    tokenMinB: number;

    constructor(data: any) {
        this.liquidityDelta = data.liquidity_delta;
        this.tokenEstA = data.token_est_a;
        this.tokenEstB = data.token_est_b;
        this.tokenMinA = data.token_min_a;
        this.tokenMinB = data.token_min_b;
    }
}



export class ClosePositionInstruction {
    instructions: TransactionInstruction[];
    additionalSigners: Keypair[];
    quote: DecreaseLiquidityQuote;
    feesQuote: CollectFeesQuote;
    rewardsQuote: number[];

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
        this.quote = new DecreaseLiquidityQuote(data.quote);
        this.feesQuote = new CollectFeesQuote(data.fees_quote);
        this.rewardsQuote = data.rewards_quote;
    }
}