import { ManagedPosition } from "@/modules/pool-manager/managed-position";
import { FloatingMenuOption, LineType } from "..";

export class ManagedPositionOptions {
    static create(managedPosition: ManagedPosition): FloatingMenuOption[] {
        const options: FloatingMenuOption[] = [];

        const closeOption = new FloatingMenuOption('Close Position', LineType.Content, () =>{ 
            const close = confirm(`Are you sure you want to close ${managedPosition.tokenA.symbol}/${managedPosition.tokenB.symbol} position with balance of $${managedPosition.balanceTotalUsd.toFixed(2)}?\n
            You will receive ${managedPosition.yieldTokenA.toFixed(2)} ${managedPosition.tokenA.symbol} and ${managedPosition.yieldTokenB.toFixed(2)} ${managedPosition.tokenB.symbol}.`);    
            if (close) managedPosition.close()
        });

        const toggleAutoRebalance = new FloatingMenuOption('Toggle Auto-Rebalance', LineType.Content, () => managedPosition.toggleAutoRebalance());
        
        options.unshift(closeOption);
        options.unshift(toggleAutoRebalance);
        
        return options;
    }
}