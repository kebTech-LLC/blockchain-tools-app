import { poolManager } from "..";
import api from "../server/api";

export class PositionSettings {
    name: string;
    rangeFactor: number;

    constructor(data: any) {
        this.name = data.name;
        this.rangeFactor = data.rangeFactor || data.range_factor * 100;
    }

    toSnakeCase() {
        return {
            name: this.name,
            range_factor: this.rangeFactor / 100
        };
    }

    async create() {
        const settings = await api.poolManager.addPositionSettings(this);
        poolManager.positionSettings.push(settings);
        return settings;
    }

    async update() {
        const updatedSettings = await api.poolManager.updatePositionSettings(this);
        poolManager.positionSettings.find((settings) => settings.name === this.name)!.rangeFactor = updatedSettings.rangeFactor;
        return updatedSettings
    }

    async get() {
        return await api.poolManager.get.positionSettings(this.name);
    }

    async delete() {
        await api.poolManager.delete.positionSettings(this.name);
        poolManager.positionSettings = poolManager.positionSettings.filter((settings) => settings.name !== this.name);
    }
}
