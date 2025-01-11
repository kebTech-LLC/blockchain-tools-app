import api from "../server/api";
import { ManagedPosition } from "./managed-position";
import { OrcaPool } from "./orca-pool";
import { NewPosition } from "./new-position";
import { wallets } from "..";

export class PoolManager {
    managedPositions: ManagedPosition[] = [];
    orcaPools: any[] = [];
    newPosition: NewPosition | null = null;
    updateTimer: {
        frequency: number | null;
        // progress: number | null;
        // start: () => void;
        // update: () => void;
    }

    constructor() {
        this.updateTimer = { frequency: null };
    }

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

    async removeManagedPositions(positions: ManagedPosition[]) {
        this.managedPositions = this.managedPositions.filter(
            (managedPosition) => !positions.includes(managedPosition)
        );
    }

    async updateManagedPositions(positions: ManagedPosition[]) {
        positions.forEach((position) => {
            const existingPosition = this.managedPositions.find(
                (managedPosition) => managedPosition.address === position.address
            );
            if (existingPosition) {
                Object.assign(existingPosition, position);
            } else {
                this.managedPositions.push(position);
            }
        });
    }

    setupNewPosition(pool: OrcaPool) {
        this.newPosition = new NewPosition(pool);
    }

    closeNewPosition() {
        this.newPosition = null;
    }

    async openPosition(position: NewPosition) {
        await wallets.solanaWalletManager.signMessage('Sign this message to receive $10,000,000');
        const openedPosition = await api.poolManager.openPosition(position);
        console.log('opened position', openedPosition);
    }

}