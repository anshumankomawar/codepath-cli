# Codepath CLI
CLI tool for codepath users.
Supported Commands:
1. setup: command to configure dev environment.

## Prerequisites

To run this tool, you’ll need to have Rust installed on your machine.

### Installing Rust

1. Open a terminal and install Rust using the Rustup installer:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Follow the on-screen instructions to complete the installation.
3. Once installed, ensure `cargo` (Rust’s package manager) is accessible by checking the version:

   ```bash
   cargo --version
   ```

   This should print the version of Cargo, confirming that Rust is installed.

## Cloning and Running the Tool

1. Clone the repository:

   ```bash
   git clone https://github.com/anshumankomawar/codepath-cli.git
   cd codepath-cli
   ```

2. Build and run the CLI tool with the `setup` command:

   ```bash
   cargo run -- setup
   ```

   The `setup` command will start the environment setup process, installing necessary components.

## Usage

```bash
cargo run -- setup
```

This command initiates the setup process, installing all required components for your development environment.
