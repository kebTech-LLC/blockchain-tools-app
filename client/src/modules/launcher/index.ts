
import { server } from "@/modules";
// import * as state from "@/state";
import { reactive, watchEffect } from "vue";

export class Launcher {
    // tasks = reactive({
    //     catalogFetch: {
    //         artist: false,
    //         band: false,
    //         public: false,
    //         connect: false,
    //     },
    //     all: false,
    //     initialItemSet: false,
    //     artistLoggedIn: false,
    // });

    // private startTime: number | null = null;
    // private elapsedTime: number = 0;
    // private timerInterval: ReturnType<typeof setInterval> | null = null;
    // public launchMessage: string = '';

    // constructor() {
    //     watchEffect(() => {
    //         if (this.allArtistFetchTasksComplete()) {
    //             server.catalogSynced = true;
    //             this.completeLaunchSequence();
    //         }
    //     });
    // }

    async start() {
        // state.views.setDimensions();
        // state.auth.initializeListeners();
        // state.navigation.initializeListeners();
        // console.log('launching app');
        // console.log('build: ', state.visitor.build); 
        // console.log('build date ', state.visitor.buildDate);

        await server.registerSocket();
        console.log('socket registered from launcher');

        // if (state.visitor.shareId) {
        //     console.log('share link detected');
        //     state.views.splashScreen = true;
        //     state.auth.showAuthScreen = false;

        //     this.startTimer();

        //     await Playlist.getPublicCatalog(state.visitor.shareId);
        //     this.setTaskStatus('public', true, 'public catalog fetched');
            
        //     state.views.tray.selectedPlaylist = state.connectCatalog.playlists[0];
        //     state.views.sort.connect.songs.current = state.views.sort.custom.find(option => option.id === 'playlist-order')!;
        //     state.views.main.view = 'connect';
            

        //     this.completeLaunchSequence();
            
        // } else {
        //     this.setTaskStatus('public', true, '');
        // }
        // watchEffect(() => {
        //     if (state.artists.activeArtist && !this.tasks.artistLoggedIn) {
        //         this.tasks.artistLoggedIn = true;    
        //     }
        // });

        // watchEffect(async () => {
        //     if (this.tasks.artistLoggedIn) {
        //         console.log('active artist detected');
        //         await clientDB.start();
                
        //         ArtistConnection.getAllForArtist()
                
        //         if (!state.visitor.shareId) state.views.splashScreen = true;

        //         this.startTimer();

        //         const artist = state.artists.activeArtist!;
        //         state.artists.updateArtists([artist]);
                
        //         Artist.getFullCatalog(artist.id).then(() => this.setTaskStatus('artist', true, 'artist catalog fetched'))
        //         // Artist.getCatalogSinceDate(artist.id).then(() => this.setTaskStatus('artist', true, 'artist catalog fetched'))
        //         Band.getArtistBands(artist.id).then(() => this.setTaskStatus('band', true, 'band catalog fetched'))
        //         Artist.getConnectCatalog(artist.id).then(() => this.setTaskStatus('connect', true, 'connect catalog fetched'))
        //     }
        // });
    }

//     startTimer() {
//         this.startTime = Date.now();
//         this.timerInterval = setInterval(() => {
//             this.updateElapsedTime();
//         }, 1000); // Update every second
//     }

//     updateElapsedTime() {
//         if (this.startTime) {
//             this.elapsedTime = Math.floor((Date.now() - this.startTime) / 1000);

//             // Update launch message based on elapsed time
//             if (this.elapsedTime < 5) {
//                 this.launchMessage = 'Loading catalog...';
//             } else if (this.elapsedTime < 10) {
//                 this.launchMessage = 'Still loading, please wait...';
//             } else {
//                 this.launchMessage = 'Taking longer than usual...';
//             }

//             console.log(this.launchMessage);
//         }
//     }

//     completeLaunchSequence() {
//         if (!this.tasks.initialItemSet) {
//             player.setInitialItem();
//             this.tasks.initialItemSet = true;
//         }

//         if (this.timerInterval) {
//             console.log("Clearing timer interval.");
//             clearInterval(this.timerInterval);
//             this.timerInterval = null; // Ensure the interval reference is cleared
//         }

//         this.tasks.all = true;
//         console.log('all launch tasks complete');
//         state.views.splashScreen = false;

//         // Final message based on total launch time
//         if (this.elapsedTime < 5) {
//             this.launchMessage = 'Launch complete!';
//         } else if (this.elapsedTime < 10) {
//             this.launchMessage = 'Launch complete! Thanks for your patience.';
//         } else {
//             this.launchMessage = 'Launch complete! We appreciate your patience.';
//         }

//         console.log(this.launchMessage);
//     }

//     setTaskStatus(task: keyof typeof this.tasks.catalogFetch, status: boolean, message: string) {
//         this.tasks.catalogFetch[task] = status;
//         console.log(message);
//     }

//     allArtistFetchTasksComplete() {
//         return Object.values(this.tasks.catalogFetch).every(status => status)
//     }

//     anyArtistFetcTasksComplete() {
//         return Object.values(this.tasks.catalogFetch).some(status => status)
//     }
}
