import { v4 as uuidv4 } from 'uuid';

const host = location.host;
const protocol = location.protocol === 'http:' ? 'http:' : 'https:';

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
  catalogSynced: boolean = false;

  constructor() {
    this.socketListeners.bind(this);
  }

  post(resource: string, operation: string, data: any): Promise<Response> {
    return this.request(HttpMethod.POST, resource, operation, data);
  }

  get(resource: string, operation: string, data?: any): Promise<Response> {
    return this.request(HttpMethod.GET, resource, operation, data);
  }

  put(resource: string, operation: string, data?: any): Promise<Response> {
    return this.request(HttpMethod.PUT, resource, operation, data);
  }

  delete(resource: string, operation: string, data?: any): Promise<Response> {
    return this.request(HttpMethod.DELETE, resource, operation, data);
  }

  request(
    requestMethod: HttpMethod,
    resource: string,
    operation: string,
    data?: any
  ): Promise<Response> {
    return new Promise((resolve, reject) => {
      // Serialize data (remove proxies, cyclical references, etc.)
      const serializedData = data ? JSON.parse(JSON.stringify(data)) : {};

      let url = `${protocol}//${host}/api/${resource}/${operation}`;
      const isQueryMethod = [HttpMethod.GET, HttpMethod.DELETE].includes(requestMethod);

      if (isQueryMethod && Object.keys(serializedData).length) {
        url += `?${new URLSearchParams(serializedData).toString()}`;
      }

      // Example headers; adjust or remove as needed
      const headers: Record<string, string> = {
        'Content-Type': 'application/json',
        // Include any additional headers you need:
        // 'Authorization': `Bearer <YOUR_TOKEN_HERE>`,
        // 'Client-Id': this.clientId ?? '',
      };

      const fetchOptions: RequestInit = {
        method: requestMethod,
        headers,
      };

      if (!isQueryMethod) {
        // Attach the data for POST/PUT
        fetchOptions.body = JSON.stringify(serializedData);
      }

      fetch(url, fetchOptions)
        .then(async (resp) => {
          const result = await resp.json();
          const responseData: Response = {
            success: result.success,
            status: resp.status.toString(),
            msg: result.msg,
            data: result.data,
          };
          resolve(responseData);
        })
        .catch((error) => {
          console.error('Fetch error:', error);
          reject(error);
        });
    });
  }

  /**
   * Example socket registration with fetch
   * Replace this with however your server provides a client ID or WebSocket token
   */
  registerSocket(): Promise<void> {
    return new Promise((resolve, reject) => {
      console.log('Registering socket...');
      // Example endpoint that returns a clientId or similar token
      fetch(`${protocol}//${host}/api/sessions/register`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      })
        .then((resp) => resp.json())
        .then((data) => {
          if (!data.clientId) {
            console.error('No clientId returned from server');
            reject('No clientId returned from server');
            return;
          }
          this.clientId = data.clientId;
          const wsProtocol = protocol.includes('https') ? 'wss://' : 'ws://';
          this.socket = new WebSocket(
            `${wsProtocol}${host}/ws?client_id=${this.clientId}`
          );
          this.socket.onerror = (e) => reject(e);
          this.socketListeners();
          resolve();
        })
        .catch((e) => reject(e));
    });
  }

  restartSocket() {
    console.log('Restarting socket...');
    if (this.socket) {
      this.socket.close();
      this.socket = undefined;
    }
    this.registerSocket().catch((err) => console.error('Socket restart failed', err));
  }

  /**
   * Attach the event listeners for this.socket
   */
  async socketListeners() {
    if (!this.socket) {
      console.error('Socket is not defined');
      return;
    }

    this.socket.onopen = () => {
      console.log('WebSocket open');
    };

    this.socket.onmessage = (m) => {
      // In a real setup, you might route messages to different handlers
      // For now, just log the data or parse it as needed
      try {
        const data = JSON.parse(m.data);
        console.log('Received WebSocket message:', data);
      } catch (err) {
        console.error('Error parsing WebSocket message:', err);
      }
    };

    this.socket.onerror = (e) => {
      console.error('WebSocket error', e);
    };

    this.socket.onclose = () => {
      this.catalogSynced = false;
      console.log('Socket closed. Attempting to reconnect every 5 seconds.');
      const intervalId = setInterval(() => {
        this.registerSocket()
          .then(() => {
            console.log('Reconnected successfully. Stopping attempts.');
            clearInterval(intervalId);
          })
          .catch((error) => {
            console.error('Reconnection attempt failed:', error);
          });
      }, 5000);
    };
  }

  /**
   * Listens for a specific response channel once, then calls your callback
   */
  socketResponseListener(responseChannel: string, callback: (data: any) => void) {
    if (!this.socket) {
      console.error('Socket is not defined');
      return;
    }
    const listener = (m: MessageEvent) => {
      try {
        const data = JSON.parse(m.data);
        if (data.response_resource === responseChannel) {
          console.log('Socket response:', data);
          callback(data);
          this.socket?.removeEventListener('message', listener);
        }
      } catch (error) {
        console.error('Error parsing socket response:', error);
      }
    };
    this.socket.addEventListener('message', listener);
  }

  /**
   * Sends a message that expects a response on a unique channel
   */
  async socketMessageWithResponseHandler(
    channel: string,
    instruction: string,
    data: any,
    callback: (data: any) => void
  ) {
    const responseChannel = uuidv4();
    this.socketResponseListener(responseChannel, (r) => {
      callback(r.data);
    });
    const msg = {
      channel,
      instruction,
      data,
      response_channel: responseChannel,
    };
    this.socket?.send(JSON.stringify(msg));
    return responseChannel;
  }

  /**
   * Sends a simple socket message without expecting a specific response
   */
  socketMessage(channel: string, instruction?: string, data?: any) {
    const msg = { channel, instruction, data };
    this.socket?.send(JSON.stringify(msg));
  }

  /**
   * Example method for sending a direct message to a specific user
   */
  messageUser(userId: string, text: string) {
    if (!this.socket) {
      console.error('Socket is not defined');
      return;
    }
    const responseChannel = uuidv4();
    const data = { channel: 'direct-message', instruction: 'new', data: text };
    const msg = {
      channel: 'message',
      instruction: 'user',
      data: {
        receiver_id: userId,
        message: data,
      },
      response_channel: responseChannel,
    };
    this.socket.send(JSON.stringify(msg));
  }

  /**
   * Example method for sending a broadcast message
   */
  broadcast(channel: string, instruction: string, data?: any) {
    if (!this.socket) {
      console.error('Socket is not defined');
      return;
    }
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
    this.socket.send(JSON.stringify(msg));
  }
}
