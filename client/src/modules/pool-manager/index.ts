import api from "../server/api";
import { ManagedPosition } from "./managed-position";
import { OrcaPool } from "./orca-pool";
import { NewPosition, NewProgrammaticPosition } from "./new-position";
import { solana } from "..";
import { OpenPositionInstruction } from "../orca/open-position-instruction";
import { ClosePositionInstruction } from "../orca/close-position-instruction";
import { Wallet } from "../solana/wallet";
import { PositionSettings } from "./position-settings";

export enum PoolType {
    Orca = 'Orca',
    Raydium = 'Raydium',
    Serum = 'Serum',
}

export class PoolManager {
    managedPositions: ManagedPosition[] = [];
    orcaPools: any[] = [];
    newPosition: NewPosition | null = null;
    positionSettings: PositionSettings[] = [];
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

    async populatePositionSettings() {
        const positionSettings = await api.poolManager.get.allPositionSettings();
        this.positionSettings.length = 0;
        this.positionSettings = positionSettings;
    }

    async removeManagedPositions(positions: ManagedPosition[]) {
        positions.forEach((position) => {
            const index = this.managedPositions.findIndex(
                (managedPosition) => managedPosition.address === position.address
            );
            if (index > -1) {
                this.managedPositions.splice(index, 1);
            }
        });
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
        const data = await api.poolManager.openPosition(position);
        const openPositionInstruction = new OpenPositionInstruction(data);
        await solana.executeInstructions(openPositionInstruction, position.wallet);
    }

    async closePosition(position: ManagedPosition) {
        const data = await api.poolManager.closePosition(position.address);
        console.log('data', data);
        const closePositionInstruction = new ClosePositionInstruction(data);
        console.log('closePositionInstruction', closePositionInstruction);
        const wallet = solana.getWallet(position.walletKey)!;
        await solana.executeInstructions(closePositionInstruction, wallet);
    }

    async openProgrammaticPosition(pool: OrcaPool) {
        try {
            const newPosition = new NewProgrammaticPosition(pool);
            const data = await api.poolManager.openProgrammaticPosition(newPosition);
            console.log('data', data);
        } catch (error) {
            console.error('error', error);
        }
     
    }

    async addPositionSettings(name: string, rangeFactor: number) {
        const positionSettings = new PositionSettings({ name, rangeFactor });
        await positionSettings.create();
    }

    async updatePositionSettings(name: string, rangeFactor: number) {
        const positionSettings = new PositionSettings({ name, rangeFactor });
        await positionSettings.update();
    }

    async deletePositionSettings(name: string) {
        await api.poolManager.delete.positionSettings(name);
    }

    async getPositionSetting(name: string) {
        return this.positionSettings.find((settings) => settings.name === name);
    }
}