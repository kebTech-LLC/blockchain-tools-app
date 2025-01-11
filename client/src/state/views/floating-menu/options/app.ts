import { FloatingMenuOption } from "..";

export class AppOptions {
    static create(): FloatingMenuOption[] {
        const options: FloatingMenuOption[] = [];
        
        // if (album.sharePermission) {
        //     const data: FloatingMenuOptionData = reactive({
        //         options: catalog.playlists.filter(playlist => !playlist.albums.find(a => a.id === album.id)),
        //         selected: '',
        //         action: () => album.addToPlaylist(data.selected)
        //     });
        //     options.unshift(new FloatingMenuOption('Add to Collection', LineType.Select, undefined, undefined, data));
        // }

        return options;
    }
}