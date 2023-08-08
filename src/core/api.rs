pub mod cascade_api {
    use crate::core::CliArgs;
    use crate::service::http_web3;
    use crate::service::websocket_web3;

    pub const PATH: &str = "src/core/api.rs";
    pub const SNIPPET: &str = r#"use crate::config::*;"#;

    /// The run function is the entry point for testing the Ethereum node
    /// The node is provided as a command line argument or if absent, the default node is used
    /// If the node is present, the is_websocket flag is set to true
    pub async fn run(args: CliArgs) {
        if is_websocket(args.node.as_str()) {
            run_websocket_test(args).await;
        } else {
            run_http_test(args).await;
        }
    }

    fn is_websocket(node: &str) -> bool {
        node.starts_with("ws://") || node.starts_with("wss://")
    }

    async fn run_websocket_test(args: CliArgs) {
        let node = args.node;
    }

    async fn run_http_test(args: CliArgs) {
        if is_default_address(args.address.as_str()) {
            run_default_test(args).await;
        }
    }

    fn is_default_address(address: &str) -> bool {
        address == "0x0"
    }

    /// The default test is run when the address is not provided
    /// The default fetches the logs from the node
    async fn run_default_test(args: CliArgs) {
        let web3Http = http_web3(args.node);
    }
}
