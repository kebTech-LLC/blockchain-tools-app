enum ServerStatus {
    Active,
    Idle,
    ShuttingDown,
}

class ServerInfo {
    id: string;
    session_id: string;
    local_ip: string;
    public_ip: string;
    port: number;
    last_heartbeat: Date;
    start_time: Date;
    cpu_usage: number;
    memory_usage: number;
    status: ServerStatus;
    max_connections: number;
    current_connections: number;
    websocket: boolean;
    redis_url: string | null;
    redis_active: boolean;

    constructor(data: any) {
        this.id = data.id;
        this.session_id = data.session_id;
        this.local_ip = data.local_ip;
        this.public_ip = data.public_ip;
        this.port = data.port;
        this.last_heartbeat = new Date(data.last_heartbeat);
        this.start_time = new Date(data.start_time);
        this.cpu_usage = data.cpu_usage;
        this.memory_usage = data.memory_usage;
        this.status = data.status;
        this.max_connections = data.max_connections;
        this.current_connections = data.current_connections;
        this.websocket = data.websocket;
        this.redis_url = data.redis_url;
        this.redis_active = data.redis_active;
    }
}