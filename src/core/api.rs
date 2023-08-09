pub mod cascade_api {
    use crate::core::CliArgs;
    use crate::log::CliLog;
    use crate::log::Logger;
    use crate::service::http_web3;
    use crate::service::websocket_web3;

    use std::env;
    use web3::types::BlockNumber;

    /// The run function is the entry point for testing the Ethereum node
    /// The node is provided as a command line argument or if absent, the default node is used
    /// If the node is present, it checks if the node is a websocket or http node
    pub async fn run(args: CliArgs) {
        if is_websocket(args.node.as_str()) {
            run_websocket_test(args).await;
        } else {
            run_http_test(args).await;
        }
    }

    /// The is_websocket function checks if the node is a websocket node
    /// It checks if the node starts with ws:// or wss://
    /// If it does, it returns true, else false
    fn is_websocket(node: &str) -> bool {
        node.starts_with("ws://") || node.starts_with("wss://")
    }

    /// The run_websocket_test function is the entry point for testing the Ethereum node
    /// It uses the Websocket transport to connect to the node
    async fn run_websocket_test(args: CliArgs) {
        let _web3_wss = websocket_web3(args.node).await;
    }

    /// The run_http_test function is the entry point for testing the Ethereum node
    /// It uses the HTTP transport to connect to the node
    async fn run_http_test(args: CliArgs) {
        if is_default_address(args.address.as_str()) {
            run_default_test(args).await;
        }
    }

    /// The is_default_address function checks if the address is the default address
    /// The default address is 0x0, which is default for the cli
    /// If the address is 0x0, it returns true, else false
    fn is_default_address(address: &str) -> bool {
        address == "0x0"
    }

    /// The default test is run when the address is not provided
    /// The default fetches the logs from the node
    async fn run_default_test(args: CliArgs) {
        let web3_http = http_web3(env::var("NODE").unwrap_or(args.node));

        let from_block = BlockNumber::Number(args.from.into());
        let to_block = BlockNumber::Number(args.to.into());

        let logs = web3_http
            .eth()
            .logs(
                web3::types::FilterBuilder::default()
                    .from_block(from_block)
                    .to_block(to_block)
                    .build(),
            )
            .await
            .expect("failed to fetch logs");

        let log = Logger {
            scope: "run_default_test".to_string(),
        };

        log.info(&format!("Logs length: {:?}", logs.len()));
    }
}
