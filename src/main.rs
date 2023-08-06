extern crate tokio;

mod config;
mod core;
mod log;
mod utils;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    core::Cli::parse();

    Ok(())
}
