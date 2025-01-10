class FloatingMenu {
    items: {
        label: string,
        icon: string | null,
        action: () => void,
    }[];
    top: number | null;
    left: number | null;
}

export class Views {
    floatingMenu: FloatingMenu = new FloatingMenu();

    
}