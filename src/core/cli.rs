use crate::core::{initialise_cli, run, test_node};

use clap::{Args, Parser, Subcommand};

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
    Test(TestArgs),
}

#[derive(Clone, Debug, Args)]
pub struct InitArgs {
    #[arg(
        long,
        short,
        default_value = "http://localhost:8545",
        help = "Node to connect to"
    )]
    pub node: String,
}

#[derive(Clone, Debug, Args)]
pub struct CliArgs {
    #[arg(long, short, default_value = "0x0", help = "Address to query")]
    pub address: String,
    #[arg(long, short, default_value = "22207815", help = "Start block")]
    pub from: u64,
    #[arg(long, short, default_value = "22207915", help = "End block")]
    pub to: u64,
    #[arg(long, short, default_value = "logs", help = "Method to run")]
    pub method: String,
    #[arg(
        long,
        short = 'T',
        default_value = "100",
        help = "Timeout for the request"
    )]
    pub timeout: u64,
    #[arg(
        long,
        short = 'p',
        default_value = "[]",
        help = "Params for the request"
    )]
    pub params: String,
}

#[derive(Clone, Debug, Args)]
pub struct TestArgs {}

pub async fn cli_main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            initialise_cli(args).await;
        }
        Commands::Run(args) => {
            run(args).await;
        }
        Commands::Test(_) => {
            test_node().await;
        }
    }
}
