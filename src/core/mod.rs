mod cli;
mod controller;

pub use cli::Cli;
pub use controller::*;

#[derive(Debug, Clone, Default)]
pub struct CliArgs {
    pub address: String,
    pub from: u64,
    pub to: u64,
    pub method: String,
    pub node: String,
}
