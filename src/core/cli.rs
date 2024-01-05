pub mod cascade_cli {
    use crate::core::{initialise_cli, run, test_cli_node};

    use clap::{Args, Parser, Subcommand};
    use serde::Serialize;

    #[derive(Parser, Debug)]
    #[command(author = "Rhaqim <anusiemj@gmail.com>", version = "0.1")]
    #[command(
        about = "cascade - a simple CLI to test nodes",
        long_about = "cascade is a simple, super CLI tool for testing arbitrium chains and functions on the nodes."
    )]
    pub struct Cli {
        #[clap(subcommand)]
        pub command: Commands,

        #[clap(flatten)]
        pub args: CliArgs,
    }

    #[derive(Subcommand, Debug)]
    pub enum Commands {
        #[command(about = "Initialize the CLI")]
        Init(InitArgs),

        #[command(about = "Run the CLI")]
        Run(CliArgs),

        #[command(about = "Test node connection")]
        Test(CliArgs),
    }

    #[derive(Clone, Debug, Args)]
    pub struct InitArgs {
        #[arg(long, short, help = "Node to connect to")]
        pub node: String,
    }

    #[derive(Clone, Debug, Args, Serialize)]
    pub struct CliArgs {
        #[arg(long, short, default_value = "", help = "Address to query")]
        #[serde(rename = "address", skip_serializing_if = "String::is_empty")]
        pub address: String,

        #[arg(long, short, default_value = "302366", help = "Start block")]
        #[serde(rename = "fromBlock")]
        pub from: u64,

        #[arg(long, short, default_value = "303366", help = "End block")]
        #[serde(rename = "toBlock")]
        pub to: u64,

        #[arg(
            long,
            short = 'p',
            default_value = "0x49d1e",
            help = "Params for the request"
        )]
        #[serde(rename = "params")]
        pub params: String,

        #[arg(
            long,
            short,
            default_value = "debug_traceBlockByNumber",
            help = "Method to run"
        )]
        pub method: String,
        #[arg(
            long,
            short = 'T',
            default_value = "100",
            help = "Timeout for the request"
        )]
        pub timeout: u64,
    }

    pub async fn cli_main() {
        let cli = Cli::parse();

        match cli.command {
            Commands::Init(args) => {
                initialise_cli(args).await;
            }
            Commands::Run(args) => {
                run(args).await;
            }
            Commands::Test(args) => {
                test_cli_node(args).await;
            }
        }
    }
}
