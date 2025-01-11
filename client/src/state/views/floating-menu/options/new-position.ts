import { NewPosition } from "@/modules/pool-manager/new-position";
import { FloatingMenuOption } from "..";

export class NewPositionOptions {
    static create(newPosition: NewPosition): FloatingMenuOption[] {
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