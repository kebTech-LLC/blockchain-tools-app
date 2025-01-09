import { ticker } from "..";

export class Ticker {
    prices: {
        'SOL': number,
        'BTC': number,
        'ETH': number,  
    }

    constructor() {
        this.prices = {
            'SOL': 0,
            'BTC': 0,
            'ETH': 0,
        };
        this.initializeCoinbaseWebSocket();
    }

    async initializeCoinbaseWebSocket() {
        const ws = new WebSocket('wss://ws-feed.exchange.coinbase.com');

        ws.onopen = () => {
            ws.send(JSON.stringify({
                type: 'subscribe',
                product_ids: ['BTC-USD', 'ETH-USD', 'SOL-USD'],
                channels: ['ticker']
            }));
            console.log('Connected to Coinbase WebSocket');
        };

        ws.onmessage = (event) => {
            const response = JSON.parse(event.data);

            if (response.type === 'ticker') {
                const { product_id, price } = response;

                switch (product_id) {
                    case 'BTC-USD':
                        ticker.prices['BTC'] = parseFloat(parseFloat(price).toFixed(2));
                        break;
                    case 'ETH-USD':
                        ticker.prices['ETH'] = parseFloat(parseFloat(price).toFixed(2));
                        break;
                    case 'SOL-USD':
                        ticker.prices['SOL'] = parseFloat(parseFloat(price).toFixed(2));
                        break;
                }
            }
        };

        ws.onclose = () => {
            console.log('Coinbase socket closed. Attempting to reconnect every 5 seconds.')
                const intervalId = setInterval(() => {
                    this.initializeCoinbaseWebSocket().then(async () => {
                        console.log('Coinbase socket reconnected successfully. Stopping attempts.');
                        clearInterval(intervalId);
                    }).catch((error) => {
                        console.error('Reconnection attempt failed:', error);
                    });
                }, 5000);
        }
    }
}