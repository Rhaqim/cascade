# Cascade - CLI Node tester

Cascade is a versatile script designed to assess the functionality of a provided Ethereum node. This script executes a series of tests and reports the results on the console. This CLI is designed to be used with linux systems.

## Installation
```bash
git clone https://github.com/rhaqim/cascade.git && cd cascade && ./install.sh
```

### Initialise CLI with node
After installing the script,  run the following command to set the node URL:

```bash
cascade --init --node <node_url>
```

## parameters

- `--node` - The URL of the Ethereum node to test
- `--timeout` - The timeout in seconds for each test
- `--help` - Prints help information
- `--version` - Prints version information
- `--verbose` - Prints additional information
- `--address` - The address to use for the tests
- `--method` - The method to use for the tests
- `--data` - The data to use for the tests

## commands

- `--init` - Initialise the CLI with the provided node URL
- `--run` - Run the tests with the provided parameters
- `--test` - Test the connection to the provided node URL

## Example

### Run Test with default parameters

```bash
cascade --test
```

This test will use the default parameters of:

- `--timeout 10`
- `--verbose false`
- `--address 0x1234567890123456789012345678901234567890`
- `--method eth_getLogs`
- `--data 0x1234567890123456789012345678901234567890`


### Run Test with custom parameters

```bash
cargo run -- --timeout 10 --verbose --address 0x1234567890123456789012345678901234567890 --method eth_getBalance --data 0x1234567890123456789012345678901234567890
```
