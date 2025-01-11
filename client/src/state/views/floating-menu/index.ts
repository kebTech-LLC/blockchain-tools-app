import { ManagedPosition } from "@/modules/pool-manager/managed-position";
import { NewPosition } from "@/modules/pool-manager/new-position";
import { OrcaPool } from "@/modules/pool-manager/orca-pool";
import { views } from "@/state";
import { AppOptions } from "./options/app";
import { PoolOptions } from "./options/pool";
import { ManagedPositionOptions } from "./options/managed-position";
import { NewPositionOptions } from "./options/new-position";


export enum FloatingMenuType {
    App = 'app',
    Pool = 'pool',
    ManagedPosition = 'managed-position',
    NewPosition = 'new-position',
}

export enum LineType {
    Content = 'content',
    Divider = 'divider',
    Heading = 'heading',
    Select = 'select',
    MultiSelect = 'multi-select',
    Space = 'space',
}

export type FloatingMenuOptionData = {
    options: any[];
    selected: any;
    action: () => void;
    showSubMenu?: boolean;
}

export class FloatingMenuOption {
    label: string;
    action: () => void;
    lineType: LineType;
    image?: string;
    data?: any;

    constructor(label: string, lineType: LineType, action?: () => void, image?: string, data?: any) {
        this.label = label;
        this.action = action || (() => {});
        this.lineType = lineType;
        this.image = image;
        this.data = data;
    }

    static divider() {
        return new FloatingMenuOption('', LineType.Divider);
    }    

    static space() {
        return new FloatingMenuOption('', LineType.Space);
    }
}

enum FloatingMenuPosition {
    TopLeft = 'top-left',
    TopRight = 'top-right',
    BottomLeft = 'bottom-left',
    BottomRight = 'bottom-right',
}

export class FloatingMenu {
    x: number;
    y: number;
    position: FloatingMenuPosition;
    menuType: FloatingMenuType;
    item?: OrcaPool | ManagedPosition | NewPosition;
    options: FloatingMenuOption[] = [];

    constructor(x: number, y: number, location: FloatingMenuPosition, menuType: FloatingMenuType, item?: OrcaPool | ManagedPosition | NewPosition) {
        this.x = x;
        this.y = y;
        this.position = location;
        this.menuType = menuType;
        this.item = item;
        this.options = this.createOptions();

        document.addEventListener('pointerdown', FloatingMenu.closeListener);
        document.addEventListener('wheel', FloatingMenu.scrollListener);
    }

    static open(event: PointerEvent, menuType: FloatingMenuType, item?: OrcaPool | ManagedPosition | NewPosition) {
        const x = event.clientX;
        const y = event.clientY;

        const documentWidth = document.body.clientWidth;
        const documentHeight = document.body.clientHeight;

        document.body.style.overflow = 'hidden';

        const locationOfClick = () => {
            const xPosition = x > documentWidth / 2 ? 'right' : 'left';
            const yPosition = y > documentHeight / 2 ? 'bottom' : 'top';
            return `${yPosition}-${xPosition}` as FloatingMenuPosition;
        };

        views.floatingMenu = new FloatingMenu(x, y, locationOfClick(), menuType, item);
    }

    static close() {
        document.removeEventListener('pointerdown', FloatingMenu.closeListener);
        document.removeEventListener('wheel', FloatingMenu.scrollListener);
        views.floatingMenu = null;
    }

    static closeListener = (event: PointerEvent) => {
        const floatingMenuElement = document.getElementById('floating-menu');
        if (floatingMenuElement && floatingMenuElement.contains(event.target as Node)) {
            return;
        }
        FloatingMenu.close();
    };

    static scrollListener = () => {
        FloatingMenu.close();
    }

    createOptions() {
        const options: FloatingMenuOption[] = [];

        const item = this.item;
        const menuType = this.menuType;
        
        switch (menuType) {
            case FloatingMenuType.App:
                const appOptions = AppOptions.create();
                options.unshift(...appOptions);
                break;
            case FloatingMenuType.Pool:
                const poolOptions = PoolOptions.create(item as OrcaPool);
                options.unshift(...poolOptions);
                break;
            case FloatingMenuType.ManagedPosition:
                const managedPositionOptions = ManagedPositionOptions.create(item as ManagedPosition);
                options.unshift(...managedPositionOptions);
                break;
            case FloatingMenuType.NewPosition:
                const newPositionOptions = NewPositionOptions.create(item as NewPosition);
                options.unshift(...newPositionOptions);
                break;
        }

        return options;
    }
}
