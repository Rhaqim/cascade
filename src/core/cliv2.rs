use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Rhaqim <anusiemj@gmail.com>", version = "0.1")]
#[command(
    about = "monster - a simple CLI to test nodes",
    long_about = "monster is a super CLI tool used to test newly created arbitrium chains. It is a simple CLI tool that can be used to test the functionality of a node."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    RunDefault(RunDefault),
    RunWithFunction(RunWithFunction),
}

#[derive(Args, Debug)]
#[command(about = "Run a default query")]
struct RunDefault {
    #[arg(long, short, default_value = "0x0", help = "Address to query")]
    address: String,
    #[arg(long, short, default_value = "earliest", help = "Start block")]
    from: u64,
    #[arg(long, short, default_value = "latest", help = "End block")]
    to: u64,
    #[arg(long, short, default_value = "logs", help = "Method to run")]
    method: String,
    #[arg(
        long,
        short,
        default_value = "http://localhost:8545",
        help = "Node to connect to"
    )]
    node: String,
    #[arg(long, short, default_value = "1000", help = "Timeout for the request")]
    timeout: u64,
}

#[derive(Args, Debug)]
#[command(about = "Run a query with a function")]
struct RunWithFunction {
    #[arg(long, short, default_value = "0x0")]
    address: String,
    #[arg(long, short, default_value = "earliest")]
    from: u64,
    #[arg(long, short, default_value = "latest")]
    to: u64,
    #[arg(long, short, default_value = "logs")]
    method: String,
    #[arg(long, short, default_value = "http://localhost:8545")]
    node: String,
}

fn cli_main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::RunDefault(args)) => {
            println!("Running with default args: {:?}", args);
        }
        Some(Commands::RunWithFunction(args)) => {
            println!("Running with function args: {:?}", args);
        }
        None => {
            println!("No command provided");
        }
    }
}
