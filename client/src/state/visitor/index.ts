import { Capacitor } from "@capacitor/core";
import { Device } from "@capacitor/device";
import utils from "@/modules/utils";

export class Visitor {
    build: string;
    buildDate: string;
    path: string;
    mode: string;
    shareId: string | undefined;
    params: URLSearchParams;
    platform: string;
    dev: boolean;
    secure: boolean;
    mobileSafari: boolean;
    online: boolean;
    native: boolean;
    host: string;
    protocol: string;
    device: any;
	mobile: boolean;
    micPermission: boolean;
    tablet: boolean;
    mouse: boolean;
    userAgent: string;

    constructor() {
        const path = location.pathname;
        const mode = path.split('/')[1];

        switch(mode) {
            case 'share': 
                this.shareId = path.split('/')[2];
                break;
        }
        this.params = new URLSearchParams(location.hash.substring(1));
        const isTauri = location.origin.includes('tauri') ? true : false;

        this.mode = mode;
        this.path = path;
        this.device = {};
        this.platform = isTauri ? 'desktop' : Capacitor.getPlatform();
        this.native = Capacitor.isNativePlatform();
        this.dev = (import.meta as any).env.VITE_APP_ENVIRONMENT === 'development' && this.platform === 'web' ? true : false;
        this.secure = true;
        this.mobileSafari = utils.isMobile() && utils.isSafari();
        this.online = navigator.onLine;
        this.protocol = this.platform == 'web' || this.platform == 'tauri' ? location.protocol : 'https:';
        this.host = location.host === 'localhost' ? 'cnctd.studio' : location.host;
        this.build = (import.meta as any).env.VITE_APP_VERSION;
        this.buildDate = (import.meta as any).env.VITE_BUILD_DATE;
		this.mobile = utils.isMobile();
        this.micPermission = false;
        this.mouse = navigator.maxTouchPoints === 0 ? true : false;
        this.userAgent = navigator.userAgent;

        Device.getInfo().then(info => {
            for (const [k, v] of Object.entries(info)) {
                const value = k == 'platform' ? this.platform : v;
                this.device[`${k}`] = value;
            }
            this.tablet = this.device?.name?.toLowerCase().includes('ipad') ? true : false;
        });

        

        window.addEventListener('online', this.updateOnlineStatus.bind(this));
        window.addEventListener('offline', this.updateOnlineStatus.bind(this));
    }

    get isChrome() {
        return this.userAgent.toLowerCase().includes('chrome');
    }

    updateOnlineStatus() {
        this.online = navigator.onLine;
        console.log('Online status updated:', this.online);
    }
}
