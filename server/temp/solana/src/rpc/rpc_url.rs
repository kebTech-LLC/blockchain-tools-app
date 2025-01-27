use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcUrl;

impl RpcUrl {
    pub fn helius() -> anyhow::Result<String> {
        let api_key = std::env::var("HELIUS_API_KEY")?;

        Ok(format!("https://rpc.helius.xyz?api-key={}", api_key))
    }

    pub fn solana_mainnet() -> String {
        "https://api.mainnet-beta.solana.com".to_string()
    }

    pub fn quicknode() -> anyhow::Result<String> {
        let api_key = std::env::var("QUICKNODE_API_KEY")?;

        Ok(format!("https://fittest-bold-card.solana-mainnet.quiknode.pro/{}/", api_key))
    }

    pub fn alchemy() -> anyhow::Result<String> {
        let api_key = std::env::var("ALCHEMY_API_KEY")?;

        Ok(format!("https://solana-mainnet.g.alchemy.com/v2/{}", api_key))
    }

    pub fn syndica() -> anyhow::Result<String> {
        let api_key = std::env::var("SYNDICA_API_KEY")?;

        Ok(format!("https://solana-mainnet.api.syndica.io/api-key/{}", api_key))
    }

    pub fn chainstack() -> anyhow::Result<String> {
        let api_key = std::env::var("CHAINSTACK_API_KEY")?;

        Ok(format!("https://solana-mainnet.core.chainstack.com/{}", api_key))
    }

    pub fn publicnode() -> String {
        "https://solana-rpc.publicnode.com".to_string()
    }

    pub fn drpc() -> String {
        "https://solana.drpc.org".to_string()
    }

    pub fn volume_priority() -> Vec<String> {
        let mut urls = vec![];
        // match Self::alchemy() {
        //     Ok(url) => urls.push(url),
        //     Err(e) => eprintln!("Error getting alchemy url: {:?}", e),
        // }
        urls.push(Self::publicnode());
        match Self::syndica() {
            Ok(url) => urls.push(url),
            Err(e) => eprintln!("Error getting syndica url: {:?}", e),
        }
        match Self::chainstack() {
            Ok(url) => urls.push(url),
            Err(e) => eprintln!("Error getting chainstack url: {:?}", e),
        }
        match Self::quicknode() {
            Ok(url) => urls.push(url),
            Err(e) => eprintln!("Error getting quicknode url: {:?}", e),
        }
        match Self::helius() {
            Ok(url) => urls.push(url),
            Err(e) => eprintln!("Error getting helius url: {:?}", e),
        }
        urls
    }

    pub fn speed_priority() -> Vec<String> {
        let mut urls = vec![];
        urls.push(Self::publicnode());
        match Self::quicknode() {
            Ok(url) => urls.push(url),
            Err(e) => eprintln!("Error getting quicknode url: {:?}", e),
        }
        match Self::helius() {
            Ok(url) => urls.push(url),
            Err(e) => eprintln!("Error getting helius url: {:?}", e),
        }
        match Self::syndica() {
            Ok(url) => urls.push(url),
            Err(e) => eprintln!("Error getting syndica url: {:?}", e),
        }
        // match Self::chainstack() {
        //     Ok(url) => urls.push(url),
        //     Err(e) => eprintln!("Error getting chainstack url: {:?}", e),
        // }
        // match Self::alchemy() {
        //     Ok(url) => urls.push(url),
        //     Err(e) => eprintln!("Error getting alchemy url: {:?}", e),
        // }
        urls
    }
}