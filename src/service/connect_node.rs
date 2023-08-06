use web3::transports::Http;
use web3::transports::WebSocket;
use web3::Web3;

pub mod connect {

    use super::*;

    pub fn http_web3(node_url: String) -> Web3<Http> {
        let http = Http::new(&node_url).unwrap();

        Web3::new(http)
    }

    pub async fn websocket_web3(node_url: String) -> Web3<WebSocket> {
        let websocket = WebSocket::new(&node_url).await.unwrap();

        Web3::new(websocket)
    }
}
