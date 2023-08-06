mod cli;
mod controller;

pub use cli::Cli;
pub use controller::*;

#[derive(Debug, Clone, Default)]
pub struct CliArgs {
    pub address: Option<String>,
    pub from: Option<u64>,
    pub to: Option<u64>,
    pub method: Option<String>,
}
