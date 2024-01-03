/// The `cascade_api` module contains functions for initializing and running the CLI.
pub mod cascade_api {
    use crate::config::Config;
    use crate::core::{CliArgs, InitArgs};
    use crate::log::CliLog;
    use crate::log::Logger;
    use crate::service::http_web3;
    use crate::service::websocket_web3;
    use std::env;
    use web3::transports::Http;
    use web3::types::BlockNumber;
    use web3::Transport;
    use web3::Web3;

    /// Initializes the CLI with the provided arguments.
    ///
    /// Uses the `Config` struct to save the node address.
    ///
    /// # Arguments
    ///
    /// * `args` - The initialization arguments.
    pub async fn initialise_cli(args: InitArgs) {
        let mut config = Config::new();

        config.node_address = args.node.clone();
        config.save();
    }

    /// Tests the connection to the saved node.
    ///  
    /// # Arguments
    ///     
    /// * `args` - The CLI arguments.
    ///     
    /// # Returns
    ///
    /// * `true` if the connection is successful, `false` otherwise.
    ///    
    /// # Panics
    ///
    /// * If the node is not a websocket node.
    /// * If the node is not a HTTP node.
    pub async fn test_cli_node() {
        let node = Config::load().node_address;

        if node.is_empty() {
            let log = Logger {
                scope: "cascade".to_string(),
            };

            log.error("Node address not initialized. Use 'init' command to set the node.");
            return;
        }

        if is_websocket(&node) {
            run_websocket_test(node).await;
        } else {
            run_http_test(node).await;
        }
    }

    /// Runs the CLI with the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The CLI arguments.
    pub async fn run(args: CliArgs) {
        // TODO: Implement the run function
    }

    /// Checks if the node is a websocket node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node URL.
    ///
    /// # Returns
    ///
    /// * `true` if the node is a websocket node, `false` otherwise.
    fn is_websocket(node: &str) -> bool {
        node.starts_with("ws://") || node.starts_with("wss://")
    }

    /// Runs the websocket test for the Ethereum node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node URL.
    async fn run_websocket_test(node: String) {
        let _web3_wss = websocket_web3(node).await;
    }

    /// Runs the HTTP test for the Ethereum node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node URL.
    async fn run_http_test(node: String) {
        let web3_http = http_web3(env::var("NODE").unwrap_or(node));

        // TODO: Implement the run_http_test function
    }

    /// Checks if the address is the default address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to check.
    ///
    /// # Returns
    ///
    /// * `true` if the address is the default address, `false` otherwise.
    fn is_default_address(address: &str) -> bool {
        address == "0x0"
    }

    /// Runs the default test when the address is not provided.
    ///
    /// # Arguments
    ///
    /// * `web3_http` - The HTTP Web3 instance.
    /// * `args` - The CLI arguments.
    async fn run_default_test(web3_http: &Web3<Http>, args: CliArgs) {
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

        if args.method == "logs" {
            log.info(&format!("Logs length: {:?}", logs.len()));
        } else {
            run_with_query_http(&web3_http, args).await;
        }
    }

    /// Runs the HTTP test with query parameters.
    ///
    /// # Arguments
    ///
    /// * `web3_http` - The HTTP Web3 instance.
    /// * `args` - The CLI arguments.
    async fn run_with_query_http(web3_http: &Web3<Http>, args: CliArgs) {
        let transport = web3_http.transport();
        let params_serde: String;

        if args.params == "[]" {
            params_serde = serde_json::from_str(&args.params).unwrap();
        } else {
            params_serde = args.params;
        }

        let get_logs = transport
            .execute(&args.method, vec![params_serde.into(), false.into()])
            .await
            .unwrap();

        let log = Logger {
            scope: "run_with_query".to_string(),
        };

        log.info(&format!("Logs length: {:?}", get_logs));
    }
}
