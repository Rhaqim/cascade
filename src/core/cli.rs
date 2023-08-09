use crate::core::run;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(author = "Rhaqim <anusiemj@gmail.com>", version = "0.1")]
#[command(
    about = "monster - a simple CLI to test nodes",
    long_about = "monster is a super CLI tool used to test newly created arbitrium chains. It is a simple CLI tool that can be used to test the functionality of a node."
)]
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
        short,
        default_value = "http://localhost:8545",
        help = "Node to connect to"
    )]
    pub node: String,
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

pub async fn cli_main() {
    let cli = CliArgs::parse();

    run(cli).await;
}
