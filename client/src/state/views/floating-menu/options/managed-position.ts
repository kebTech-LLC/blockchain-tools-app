import { ManagedPosition } from "@/modules/pool-manager/managed-position";
import { FloatingMenuOption, LineType } from "..";

export class ManagedPositionOptions {
    static create(managedPosition: ManagedPosition): FloatingMenuOption[] {
        const options: FloatingMenuOption[] = [];

        const closeOption = new FloatingMenuOption('Close Position', LineType.Content, () => managedPosition.close());
        
        options.unshift(closeOption);
        
        return options;
    }
}