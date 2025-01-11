import { poolManager } from "..";
import { OrcaPool } from "./orca-pool";

export class NewPosition {
    pool: OrcaPool;
    lowerRange: number;
    UpperRange: number;

    constructor(pool: OrcaPool) {
        this.pool = pool;
    }
}