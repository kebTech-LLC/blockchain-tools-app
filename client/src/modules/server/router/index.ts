import { Artist, Image, Recording } from "@/models";
import { server } from "@/modules";
import { artists, catalog, connectCatalog } from "@/state";
import { PendingArtistConnection } from "@/state/artists";

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
        
        if (this.channel.includes(':')) {
            const [channel, _id] = this.channel.split(':');
            switch (channel)  {
                case 'artist':
                    switch (this.instruction) {
                        case 'updated-artist':
                            const updatedArtist = new Artist(this.data);
                            artists.updateArtists([updatedArtist]);
                            break;
                        default:
                            console.log('no route for instruction', this.instruction);
                    }
                    break;
                case 'image':
                    switch (this.instruction) {
                        case 'updated-image':
                            const updatedImage = new Image(this.data);
                            connectCatalog.updateImages([updatedImage]);
                            break;
                        default:
                            console.log('no route for instruction', this.instruction);
                    }
                    break;

                case 'album':
                    switch (this.instruction) {
                        case 'updated-album':
                            console.log('updated album', this.data);
                            break;
                        default:
                            console.log('no route for instruction', this.instruction);
                    }
                    break
            }
        }

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
            
            case 'artist-connections': 
                switch(this.instruction) {
                    case 'accepted-connection': 
                        const newArtist = new Artist(this.data.artist);
                        artists.updateArtists([newArtist]);
                        const updatedArtistConnection = new PendingArtistConnection(this.data.connection);
                        artists.removePendingConnection(updatedArtistConnection);
                        break;
                    case 'new-connection':
                        const newConnection = new PendingArtistConnection(this.data);
                        artists.updatePendingConnections([newConnection]);
                        break;
                }
                break;
            case 'images':
                switch (this.instruction) {
                    case 'new-image':
                        const newImage = new Image(this.data);
                        catalog.updateImages([newImage]);
                        break;
                    case 'updated-image':
                        const updatedImage = new Image(this.data);
                        catalog.updateImages([updatedImage]);
                        break;
                    default:
                        console.log('no route for instruction', this.instruction);
                }
                break;

            case 'albums': 
                switch (this.instruction) {
                    case 'new-album':
                        console.log('new album', this.data);
                        break;
                    case 'updated-album':
                        console.log('updated album', this.data);
                        break;
                    default:
                        console.log('no route for instruction', this.instruction);
                }
                break
                    
            case 'recordings':
                switch (this.instruction) {
                    case 'new-recording':
                        const newRecording = new Recording(this.data);
                        catalog.updateRecordings([newRecording]);
                        break;
                    case 'updated-recording':
                        const updatedRecording = new Recording(this.data);
                        catalog.updateRecordings([updatedRecording]);
                        break;
                    default:
                        console.log('no route for instruction', this.instruction);
                }
                break;

            default:
                console.log('no route for channel', this.channel);
        }
    }
}
