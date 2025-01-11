import { OrcaPool } from "@/modules/pool-manager/orca-pool";
import { FloatingMenuOption, LineType } from "..";
import { poolManager } from "@/modules";

export class PoolOptions {
    static create(orcaPool: OrcaPool): FloatingMenuOption[] {
        const options: FloatingMenuOption[] = [];
        
        const openOption = new FloatingMenuOption('Open New Position', LineType.Content, () => poolManager.setupNewPosition(orcaPool));

        options.unshift(openOption);

        return options;
    }
}