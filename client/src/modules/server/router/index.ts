import { liquidityPools, server } from "@/modules";
import { ManagedPosition } from "@/modules/liquidity-pools/managed-position";

export class IncomingSocketMessage {
    channel: string;
    success: boolean;
    instruction: string;
    data: any;

    constructor(data: any) {
        this.channel = data.channel;
        this.success = data.success
        this.instruction = data.instruction;
        this.data = data.data;
    }

    route() {
        console.log('routing', this.channel, this.instruction, this.data);

        switch (this.channel) {
            // case 'session': {
            //     switch (this.instruction) {
            //         case 'catalog-synced': 
            //             server.catalogSynced = this.data;
            //             break;
            //     }
            //     break;
            // }
            case 'server-info': 
                switch (this.instruction) {
                    case 'heartbeat':
                        server.info = this.data;
                        break;
                }
                break;

            case 'socket':
                switch (this.instruction) {
                    case 'client-info':
                        console.log('client info', this.data);
                        break;
                }
                break;

            case 'managed-position':
                switch(this.instruction) {
                    case 'update':
                        const managedPosition = new ManagedPosition(this.data);
                        const existingPosition = liquidityPools.managedPositions.find(position => position.address === managedPosition.address);
                        if (existingPosition) {
                            Object.assign(existingPosition, managedPosition);
                        } else {
                            liquidityPools.managedPositions.push(managedPosition);
                        }
                        break;
                }
                break;
            default:
                console.log('no route for channel', this.channel);
        }
    }
}
