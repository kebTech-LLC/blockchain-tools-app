import { Keypair, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Buffer } from 'buffer';

class IncreaseLiquidityQuote {
    liquidityDelta: number;
    tokenEstA: number;
    tokenEstB: number;
    tokenMaxA: number;
    tokenMaxB: number;

    constructor(data: any) {
        this.liquidityDelta = data.liquidity_delta;
        this.tokenEstA = data.token_est_a;
        this.tokenEstB = data.token_est_b;
        this.tokenMaxA = data.token_max_a;
        this.tokenMaxB = data.token_max_b;
    }
}

export class OpenPositionInstruction {
    positionMint: PublicKey;
    quote: IncreaseLiquidityQuote;
    instructions: TransactionInstruction[];
    additionalSigners: Keypair[];
    initializationCost: number;

    constructor(data: any) {
        this.positionMint = new PublicKey(data.position_mint);
        this.quote = new IncreaseLiquidityQuote(data.quote);
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
        this.additionalSigners = data.additional_signers.map((signer: string) => Keypair.fromSecretKey(Uint8Array.from(Buffer.from(signer, 'base64'))));
        this.initializationCost = data.initialization_cost;
    }
}

// const data = {
//     "additional_signers": [
//         "5iY83T6CVcLzTzLJKR8wt4yy5Ej713ZjTXSrrEeMfo9SCv2oSBTmGTXAV5HGr3CbAKKAbcgGMoQ6rjYFfo8KP9MG",
//         "2wRqbiSXxgBSJWJnDJanEZ6cx6MRC9Tgk1MGikavbCCenfdLVX4Pjy2t4ZmdiytHn4KmrVYkiFng11JCrHXnA3Yv"
//     ],
//     "initialization_cost": 0,
//     "instructions": [
//         {
//             "accounts": [
//                 {
//                     "is_signer": true,
//                     "is_writable": true,
//                     "pubkey": "312yxT6PFcauztXCfG5jNqcRXqMDCm9HeLBJwbaHL6kH"
//                 },
//                 {
//                     "is_signer": true,
//                     "is_writable": true,
//                     "pubkey": "7fugSh5V2J91r1jMCt4zjithHuShHYTwe5wCkgunz8aE"
//                 }
//             ],
//             "data": [
//                 0,
//                 0,
//                 0,
//                 0,
//                 243,
//                 29,
//                 31,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 165,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 6,
//                 221,
//                 246,
//                 225,
//                 215,
//                 101,
//                 161,
//                 147,
//                 217,
//                 203,
//                 225,
//                 70,
//                 206,
//                 235,
//                 121,
//                 172,
//                 28,
//                 180,
//                 133,
//                 237,
//                 95,
//                 91,
//                 55,
//                 145,
//                 58,
//                 140,
//                 245,
//                 133,
//                 126,
//                 255,
//                 0,
//                 169
//             ],
//             "program_id": "11111111111111111111111111111111"
//         },
//         {
//             "accounts": [
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "7fugSh5V2J91r1jMCt4zjithHuShHYTwe5wCkgunz8aE"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "So11111111111111111111111111111111111111112"
//                 }
//             ],
//             "data": [
//                 18,
//                 29,
//                 185,
//                 157,
//                 106,
//                 154,
//                 82,
//                 154,
//                 117,
//                 22,
//                 172,
//                 121,
//                 14,
//                 38,
//                 189,
//                 152,
//                 132,
//                 200,
//                 3,
//                 213,
//                 65,
//                 178,
//                 125,
//                 154,
//                 208,
//                 25,
//                 51,
//                 84,
//                 247,
//                 115,
//                 6,
//                 139,
//                 154
//             ],
//             "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
//         },
//         {
//             "accounts": [
//                 {
//                     "is_signer": true,
//                     "is_writable": true,
//                     "pubkey": "312yxT6PFcauztXCfG5jNqcRXqMDCm9HeLBJwbaHL6kH"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "312yxT6PFcauztXCfG5jNqcRXqMDCm9HeLBJwbaHL6kH"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "E7DRPmWTmhuMM8vXMaj9yfDQyzQSqcrRAAtwZsrkGQNh"
//                 },
//                 {
//                     "is_signer": true,
//                     "is_writable": true,
//                     "pubkey": "HBpq8dLTopvcYCRNdaofKDm3umgiutCpVYM5rwjzr74i"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "G2oDJfVdrBwCtnRiXZ2kAwtfcMcjUNj7pJ7Bne1tUrHK"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "11111111111111111111111111111111"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"
//                 }
//             ],
//             "data": [
//                 212,
//                 47,
//                 95,
//                 92,
//                 114,
//                 102,
//                 131,
//                 250,
//                 224,
//                 188,
//                 255,
//                 255,
//                 172,
//                 189,
//                 255,
//                 255,
//                 1
//             ],
//             "program_id": "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
//         },
//         {
//             "accounts": [
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
//                 },
//                 {
//                     "is_signer": true,
//                     "is_writable": false,
//                     "pubkey": "312yxT6PFcauztXCfG5jNqcRXqMDCm9HeLBJwbaHL6kH"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "E7DRPmWTmhuMM8vXMaj9yfDQyzQSqcrRAAtwZsrkGQNh"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "G2oDJfVdrBwCtnRiXZ2kAwtfcMcjUNj7pJ7Bne1tUrHK"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "So11111111111111111111111111111111111111112"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": false,
//                     "pubkey": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "7fugSh5V2J91r1jMCt4zjithHuShHYTwe5wCkgunz8aE"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "gzAsGwTMK8oonfLtCQbGg6ey3MLzX9xRRVK4en3mnAh"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "EUuUbDcafPrmVTD5M6qoJAoyyNbihBhugADAxRMn5he9"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "2WLWEuKDgkDUccTpbwYp1GToYktiSB1cXvreHUwiSUVP"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "63vsBPDtySAoA1tEdaevWJbDYCQ3UNPRa1Lkf93WPZc6"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "63vsBPDtySAoA1tEdaevWJbDYCQ3UNPRa1Lkf93WPZc6"
//                 }
//             ],
//             "data": [
//                 133,
//                 29,
//                 89,
//                 223,
//                 69,
//                 238,
//                 176,
//                 10,
//                 147,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 3,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 2,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0
//             ],
//             "program_id": "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
//         },
//         {
//             "accounts": [
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "7fugSh5V2J91r1jMCt4zjithHuShHYTwe5wCkgunz8aE"
//                 },
//                 {
//                     "is_signer": false,
//                     "is_writable": true,
//                     "pubkey": "312yxT6PFcauztXCfG5jNqcRXqMDCm9HeLBJwbaHL6kH"
//                 },
//                 {
//                     "is_signer": true,
//                     "is_writable": false,
//                     "pubkey": "312yxT6PFcauztXCfG5jNqcRXqMDCm9HeLBJwbaHL6kH"
//                 }
//             ],
//             "data": [
//                 9
//             ],
//             "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
//         }
//     ],
//     "position_mint": "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE",
//     "quote": {
//         "liquidity_delta": 147,
//         "token_est_a": 2,
//         "token_est_b": 1,
//         "token_max_a": 3,
//         "token_max_b": 2
//     }
// }