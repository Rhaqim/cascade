extern crate tokio;

mod config;
mod core;
mod log;
mod report;
mod service;
mod utils;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    core::cli_main().await;

    Ok(())
}
