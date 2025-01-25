import { OrcaPool } from "@/modules/pool-manager/orca-pool";
import { FloatingMenuOption, LineType } from "..";
import { poolManager } from "@/modules";

export class PoolOptions {
    static create(orcaPool: OrcaPool): FloatingMenuOption[] {
        const options: FloatingMenuOption[] = [];
        
        const openOption = new FloatingMenuOption('Open New Position', LineType.Content, () => poolManager.setupNewPosition(orcaPool));
        const openProgrammaticPosition = new FloatingMenuOption('Open Programmatic Position', LineType.Content, () => {
            const open = confirm(`Are you sure you want to open a programmatic position for ${orcaPool.tokenA.symbol}/${orcaPool.tokenB.symbol}?\n`);
            if (open) poolManager.openProgrammaticPosition(orcaPool)
        });

        options.unshift(openProgrammaticPosition)
        options.unshift(openOption);

        return options;
    }
}