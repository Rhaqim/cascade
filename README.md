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

1. Run Test with default parameters:

```bash
cargo run
```

This test will use the default parameters of:

- `--node http://localhost:8545`
- `--timeout 10`
- `--verbose false`
- `--address 0x1234567890123456789012345678901234567890`
- `--method eth_getLogs`
- `--data 0x1234567890123456789012345678901234567890`

2. Run Test with custom parameters:

```bash
cargo run -- --node https://mainnet.infura.io/v3/1234567890 --timeout 10 --verbose --address 0x1234567890123456789012345678901234567890 --method eth_getBalance --data 0x1234567890123456789012345678901234567890
```
