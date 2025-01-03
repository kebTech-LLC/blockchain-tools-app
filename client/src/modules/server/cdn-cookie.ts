export class CookieManager {
    private signedCookieString: string;

    constructor(signedCookieString: string) {
        this.signedCookieString = signedCookieString;
    }

    /**
     * Set the cookie in the browser.
     */
    public setCookie(): void {
        const cookieData = JSON.parse(this.signedCookieString.split('.')[0]); // Parse the JSON part of the cookie
        console.log('cookie data', cookieData); 
        const { key, expiration, path, domain, secure } = cookieData;

        // Set the expiration time
        const expires = new Date(expiration * 1000).toUTCString();

        // Construct the full cookie string including the signature
        let cookieString = `${key}=${this.signedCookieString}; Path=${path}; Domain=${domain}; Expires=${expires};`;

        if (secure) {
            cookieString += ' Secure;';
        }

        // Set the cookie in the browser
        document.cookie = cookieString;
        console.log(`Cookie set: ${cookieString}`);
    }

    /**
     * Check if the cookie has expired.
     */
    public isExpired(): boolean {
        const cookieData = JSON.parse(this.signedCookieString.split('.')[0]);
        const now = Math.floor(Date.now() / 1000); // Current time in seconds
        return now > cookieData.expiration;
    }
}
