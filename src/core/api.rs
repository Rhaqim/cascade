/// The `cascade_api` module contains functions for initializing and running the CLI.
pub mod cascade_api {
    use crate::config::Config;
    use crate::core::{CliArgs, InitArgs};
    use crate::report::{Report, ReportData, ReportHeader};

    use crate::error;
    use crate::service::RpcRequest;

    // use web3::types::BlockNumber;

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

    /// Runs the CLI with the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The CLI arguments.
    pub async fn run(args: CliArgs) {
        let header = ReportHeader {
            node: "node".to_string(),
            args: args.clone(),
        };

        let request = RpcRequest::new(None);

        // let from_block = BlockNumber::Number(args.from.into());
        // let to_block = BlockNumber::Number(args.to.into());

        // let params = serde_json::json!({
        //     "fromBlock": from_block,
        //     "toBlock": to_block,
        //     "address": format!("0x{}", args.address),
        // });

        let mut response = request
            .make_request(&args.method, args.params.clone())
            .await;

        if args.depth > 1 {
            response = request
                .make_multiple_requests(&args.method, args.params.clone(), args.depth as usize)
                .await;
        }

        match response {
            Ok(_) => {
                let data = [ReportData {
                    success: true,
                    error: None,
                    duration: 0,
                    result: Some("Success".to_string()),
                }]
                .to_vec();

                let mut report = Report::new(header);

                report.add_data(data[0].clone());

                report.display();
            }
            Err(e) => {
                error!("Error: {:?}", e);
            }
        }
    }
}
