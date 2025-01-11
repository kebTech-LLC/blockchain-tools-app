import { poolManager, wallets } from "..";
import { OrcaPool } from "./orca-pool";

enum PoolType {
    Orca,
    Raydium,
    Serum
}

export class NewPosition {
    walletKey: string;
    walletType: string;
    poolType: PoolType;
    rangeLower: number;
    rangeUpper: number;
    pool: OrcaPool;

    constructor(pool: OrcaPool, walletKey: string) {
        this.walletKey = walletKey;
        this.walletType = wallets.solanaWalletManager.getWalletType(walletKey) || '';
        this.poolType = PoolType.Orca;
        this.rangeLower = 0;
        this.rangeUpper = 0;
        this.pool = pool;
    }

    open() {
        poolManager.openPosition(this);
    }
}