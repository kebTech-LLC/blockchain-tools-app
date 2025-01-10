import api from "../server/api";
import { ManagedPosition } from "./managed-position";

export class PoolManager {
    managedPositions: ManagedPosition[] = [];
    orcaPools: any[] = [];

    async populateOrcaPools(limit?: number) {
        this.orcaPools = await api.poolManager.get.orcaPools(limit);
    }

    async populateManagedPositions() {
        const managedPositions = await api.poolManager.get.allPositions();
        this.managedPositions.length = 0;
        this.managedPositions = managedPositions.map((position: any) => new ManagedPosition(position));
    }
}