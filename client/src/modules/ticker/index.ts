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

    initializeCoinbaseWebSocket() {
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
                        ticker.prices['BTC'] = parseFloat(price);
                        break;
                    case 'ETH-USD':
                        ticker.prices['ETH'] = parseFloat(price);
                        break;
                    case 'SOL-USD':
                        ticker.prices['SOL'] = parseFloat(price);
                        break;
                }
            }
        };
        
    }
}