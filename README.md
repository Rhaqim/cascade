# Ethereum CLI node tester

This is a simple script to test a provided Ethereum node. It will run a series of tests and output the results to the console.

## Usage

1. Clone this repository
2. Run `cargo build`
3. Run `cargo run -- --help` to see the available options

## parameters

- `--node` - The URL of the Ethereum node to test
- `--timeout` - The timeout in seconds for each test
- `--help` - Prints help information
- `--version` - Prints version information
- `--verbose` - Prints additional information
- `--address` - The address to use for the tests
- `--method` - The method to use for the tests
- `--data` - The data to use for the tests

## Example

1. Run `cargo run -- --node https://mainnet.infura.io/v3/your-api-key --timeout 10`
