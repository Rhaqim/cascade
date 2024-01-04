#!/bin/bash

# Check if Rust is already installed
if ! command -v cargo &> /dev/null; then
    # Install Rust using rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Add cargo to PATH
    source $HOME/.cargo/env
fi

# Install the Rust CLI dependencies
# Add your dependencies here

# Check if necessary environment variables are set
# if [ -z "$NODE_ADDRESS" ]; then
#     read -p "Enter the Node Address: " NODE_ADDRESS
#     export NODE_ADDRESS
#     echo "export NODE_ADDRESS=$NODE_ADDRESS" >> $HOME/.bashrc  # Adjust this line based on the user's shell profile
# fi

# Build and install the Rust CLI
cargo build --release
cargo install --path .

# Print success message
echo "Rust CLI installed successfully!"

# Run the help command
$ cascade --help
