use crate::config::*;
use crate::core::{cli_controller::run, CliArgs};
use clap::{Arg, ArgAction, Command};

pub struct Cli;

impl Cli {
    pub async fn parse() {
        let matches = Command::new("monster")
            .version("0.1.0")
            .author("Rhaqim <anusiemj@gmail.com>")
            .about("Ethereum test CLI")
            .arg(
                Arg::new("node")
                    .help("Node to connect to")
                    .long("node")
                    .short('n')
                    .number_of_values(1)
                    .value_name("NODE")
                    .required(true),
            )
            .arg(
                Arg::new("address")
                    .help("Add a contract to the database")
                    .long("address")
                    .short('a')
                    .number_of_values(1)
                    .value_name("ADDRESS")
                    .required(false),
            )
            .arg(
                Arg::new("from")
                    .help("Start indexing from this block")
                    .long("from")
                    .short('f')
                    .number_of_values(1)
                    .value_name("FROM_BLOCK")
                    .required(false),
            )
            .arg(
                Arg::new("to")
                    .help("Stop indexing at this block")
                    .long("to")
                    .short('t')
                    .number_of_values(1)
                    .value_name("TO_BLOCK")
                    .required(false),
            )
            .arg(
                Arg::new("method")
                    .help("Method to run")
                    .long("method")
                    .short('m')
                    .number_of_values(1)
                    .value_name("METHOD")
                    .required(false),
            )
            .arg(
                Arg::new("run")
                    .help("Run the indexer")
                    .long("run")
                    .short('r')
                    .action(ArgAction::Set),
            )
            .get_matches();

        let binding = DEFAULT_ADDRESS.to_string();
        let address = matches.get_one::<String>("address").unwrap_or(&binding);
        let from = matches
            .get_one::<u64>("from")
            .unwrap_or(&DEFAULT_FROM_BLOCK);
        let to = matches.get_one::<u64>("to").unwrap_or(&DEFAULT_TO_BLOCK);
        let binding = DEFAULT_METHOD.to_string();
        let method = matches.get_one::<String>("method").unwrap_or(&binding);
        let binding = DEFAULT_NODE.to_string();
        let node = matches.get_one::<String>("node").unwrap_or(&binding);

        let cli_args = CliArgs {
            address: address.to_string(),
            from: from.to_owned(),
            to: to.to_owned(),
            method: method.to_string(),
            node: node.to_string(),
        };

        run(cli_args).await;
    }
}
