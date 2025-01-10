import api from "../server/api";
import { ManagedPosition } from "./managed-position";
import { OrcaPool } from "./orca-pool";

export class PoolManager {
    managedPositions: ManagedPosition[] = [];
    orcaPools: any[] = [];

    async populateOrcaPools(limit?: number) {
        const orcaPools = await api.poolManager.get.orcaPools(limit);
        this.orcaPools.length = 0;
        this.orcaPools = orcaPools.map((pool: any) => new OrcaPool(pool));
        
    }

    async populateManagedPositions() {
        const managedPositions = await api.poolManager.get.allPositions();
        this.managedPositions.length = 0;
        this.managedPositions = managedPositions.map((position: any) => new ManagedPosition(position));
    }
}