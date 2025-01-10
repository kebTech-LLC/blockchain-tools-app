import { v4 as uuidv4 } from 'uuid';
// import { visitor } from '@/state'
import { IncomingSocketMessage } from './router';
import api from './api';
import { Http } from '@capacitor-community/http';
import { poolManager } from '..';


const host = location.host;
const protocol = location.protocol === 'http:'? 'http:': 'https:';


enum HttpMethod {
    GET = 'GET',
    POST = 'POST',
    PUT = 'PUT',
    DELETE = 'DELETE',
}

export interface Response {
    success: boolean;
    status: string;
    msg?: string;
    data?: any;
}

export interface Message {
    resource: string;
    operation: string;
    user_id: string;
    data: any;

    constructor(resource: string, operation: string, user_id: string, data: any): Message;
}

export interface ClientInfo {
    client_id: string;
    user_id: string;
    authenticated: boolean;
    subscriptions: string[];
    connected: boolean;
    server_id: string;
    data: any;
}


export class Server {
    socket: WebSocket | undefined;
    clientId: string | undefined;
    info: ServerInfo | undefined;

    constructor() {
        this.socketListeners.bind(this);
    }
    
    post(resource: string, operation: string, data: any): Promise<Response> {
        return new Promise((ok, err) => {
            this.request(HttpMethod.POST, resource, operation, data)
                .then(response => ok(response))
                .catch(e => err(e))
        });
    }

    get(resource: string, operation: string, data?: any): Promise<Response> {
        return new Promise((ok, err) => {
            this.request(HttpMethod.GET, resource, operation, data)
                .then(response => ok(response))
                .catch(e => err(e))
        });
    }
    
    put(resource: string, operation: string, data?: any): Promise<Response> {
        return new Promise((ok, err) => {
            this.request(HttpMethod.PUT, resource, operation, data)
                .then(response => ok(response))
                .catch(e => err(e))
        });
    }

    delete(resource: string, operation: string, data?: any): Promise<Response> {
        return new Promise((ok, err) => {
            this.request(HttpMethod.DELETE, resource, operation, data)
                .then(response => ok(response))
                .catch(e => err(e))
        });
    }

    request(requestMethod: HttpMethod, resource: string, operation: string, data?: any): Promise<Response> {
        return new Promise((ok, err) => {
            // Serialize data by removing Proxy wrappers and any non-serializable elements
            const serializedData = data ? JSON.parse(JSON.stringify(data)) : {};
    
            let url = `${protocol}//${host}/api/${resource}/${operation}`;
            url = [HttpMethod.GET, HttpMethod.DELETE].includes(requestMethod) ? url + '?' + new URLSearchParams(serializedData) : url;
    
            const options = {
                url,
                method: requestMethod,
                headers: {
                    'Content-Type': 'application/json',
                    // 'Authorization': auth.userSession?.access_token || '',
                    'Client-Id': this.clientId || '',
                },
                data: [HttpMethod.POST, HttpMethod.PUT].includes(requestMethod) ? serializedData : undefined,
            };
    
            Http.request(options)
                .then(response => {
                    const responseData = response.data;
                    const res: Response = {
                        success: responseData.success,
                        status: response.status.toString(),
                        msg: responseData.msg,
                        data: responseData.data,
                    };
                    ok(res);
                })
                .catch(e => {
                    console.error('error', e);
                    err(e);
                });
        });
    }
    
    

    registerSocket(): Promise<void> {
        poolManager.populateManagedPositions();
        console.log('Registering application socket');
        return new Promise((ok, err) => {
            api.sessions.register()
                .then(async clientId => {
                    if (!clientId) {
                        console.log('No client id returned from server');
                        err('No client id returned from server');
                    }
                    this.clientId = clientId;
                    const wsProtocol = protocol.includes('s')? 'wss://' : 'ws://';
                    this.socket = new WebSocket(wsProtocol + host + '/ws?client_id=' + this.clientId);
                    this.socket.onerror = (e) => err(e);
                    await this.socketListeners();
                    ok()
                })
                .catch(e => err(e));
        });
    }

    // registerSocket(): Promise<void> {
    //     return new Promise((resolve, reject) => {
    //         if (this.socket && this.socket.readyState === WebSocket.OPEN) {
    //             resolve();
    //             return;
    //         }
    
    //         console.log('registering socket')
    //         const subscriptions = ['server-info'];
    //         // if (auth.userAccount?.isAdmin) subscriptions.push('client-info');
    //         const unauthString = !auth.loggedIn && visitor.mode === 'share' ? '&unauth_id=' + visitor.shareId : '';
    //         console.log('unauth string', unauthString);
    //         const registrationUrl = `${protocol}//${host}/register?subscriptions=${subscriptions}` + unauthString;
    //         fetch(registrationUrl, { 
    //             // headers: {
    //             //     'Content-Type': 'application/json',
    //             //     ...(auth.userSession?.access_token && {
    //             //         'Authorization': auth.userSession.access_token
    //             //     }),
    //             // },
    //             mode: 'cors',
    //         })
    //         .then(r => r.json())
    //         .then(async r => {
    //             const clientId = r.data;
    //             console.log('server assigned client id', clientId);
    //             this.clientId = clientId;
    //             const wsProtocol = protocol.includes('s')? 'wss://' : 'ws://';
    //             this.socket = new WebSocket(wsProtocol + host + '/ws?client_id=' + clientId);
    //             this.socket.onerror = (e) => reject(e);
    //             await this.socketListeners();
    //         })
    //         .then(resolve)
    //         .catch(reject);
    //     });
    // }

    // async authenticateSocket() {
    //     return await api.socket.update.authenticate(this.clientId!, auth.user!.id)
    // }

    // async updateSocketUserId(userId: string) {
    //     return await api.socket.update.userId(this.clientId!, userId)
    // }

    restartSocket() {
        console.log('Restarting application socket');
        if (this.socket) {
            this.socket.close();
            this.socket = undefined;
        }
        this.registerSocket();
    }

    async socketListeners() {
        if (this.socket) {
            const responseChannel = uuidv4();
            this.socket.onopen = () => {
                this.socket!.onmessage = m => {
                    const message = new IncomingSocketMessage(JSON.parse(m.data))
                    message.route();
                }
            } 
            this.socket.onerror = e => console.error('socket error', e)
            this.socket.onclose = () => {
                console.log('socket closed. Attempting to reconnect every 5 seconds.')
                    const intervalId = setInterval(() => {
                        this.registerSocket().then(async () => {
                            console.log('Reconnected successfully. Stopping attempts.');
                            clearInterval(intervalId);
                        }).catch((error) => {
                            console.error('Reconnection attempt failed:', error);
                        });
                    }, 5000);
            }
        } else {
            console.error('socket is not defined')
        }
    }

    socketResponseListener(responseChannel: string, callback: (data: any) => void) {
        if (this.socket) {
            const listener = this.socket.onmessage = m => {
                const data = JSON.parse(m.data);
                if (data.response_resource === responseChannel) {
                    console.log('socket response', data);
                    callback(data);
                    this.socket?.removeEventListener('message', listener);
                }
            }
        } else {
            console.error('socket is not defined')
        }
    }

    async socketMessageWithResponseHandler(channel: string, instruction: string, data: any, callback: (data: any) => void) {
        const responseChannel = uuidv4();
        this.socketResponseListener(responseChannel, r => {
            callback(r.data);
        });
        const msg = { 
            channel, 
            instruction, 
            data, 
            response_channel: responseChannel 
        };
        this.socket?.send(JSON.stringify(msg));
        return responseChannel;
    }

    socketMessage(channel: string, instruction?: string, data?: any) {
        const msg = { 
            channel, 
            instruction, 
            data, 
        };
        this.socket?.send(JSON.stringify(msg));
    }

    messageUser(userId: string, text: string) {
        const responseChannel = uuidv4();
        const data = { channel: 'direct-message', instruction: 'new', data: text };
        const msg = { 
            channel: 'message', 
            instruction: 'user', 
            data: {
                receiver_id: userId, 
                message: data,
            },
            response_channel: responseChannel 
        };
        this.socket?.send(JSON.stringify(msg));
    }

    broadcast(channel: string, instruction: string, data?: any) {
        const responseChannel = uuidv4();
        const msg = { 
            channel: 'message', 
            instruction: 'broadcast', 
            data: {
                channel, 
                instruction, 
                data,
            }, 
            response_channel: responseChannel,
        };
        this.socket?.send(JSON.stringify(msg));
    }
    
}

