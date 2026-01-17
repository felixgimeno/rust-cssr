# Rust-CSSR

This repository contains a modern Rust implementation of the [Causal-State Splitting Reconstruction (CSSR)](http://bactra.org/CSSR/) algorithm. CSSR is an algorithm for building recursive hidden Markov models from discrete-valued time series and other discrete sequential data.

## Building

To build the project, you need to have Rust and Cargo installed. You can find instructions for installing them on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can build the project by running the following command in the root of the repository:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/rust-cssr`.

## Usage

The command-line interface allows you to run the CSSR algorithm on a data file. The data file should contain a sequence of unsigned integers, with one integer per line.

### Command-line arguments

- `-f, --file <FILE>`: Path to the input data file.
- `-m, --max-history <MAX_HISTORY>`: Maximum history length to consider (default: 10).
- `-a, --alpha <ALPHA>`: Significance level for the chi-square test (default: 0.05).

### Example

To run the CSSR algorithm on a data file named `data.txt` with a maximum history length of 5 and a significance level of 0.01, you would run the following command:

```bash
./target/release/rust-cssr --file data.txt --max-history 5 --alpha 0.01
```

## Testing

To run the tests for this project, you can use the following command:

```bash
cargo test
```

## Developer Notes

This section contains a summary of the key learnings and conventions used in this repository.

- **Deep Planning Mode**: Before making any changes, it is important to start in a deep planning mode. This involves asking clarifying questions to fully understand requirements and goals, confirming assumptions, and only creating a plan once there is absolute certainty. Use `request_user_input` for this interaction.
- **Robust Code**: The user prefers robust code with comprehensive tests and proper error handling (avoiding `.unwrap()`).
- **Project Structure**: This repository is a modern Rust implementation of the Causal State Splitting Reconstruction (CSSR) algorithm. The project is structured as a Rust workspace with a library crate (`src/lib.rs`) for the core logic and a binary crate (`src/bin/main.rs`) for the command-line interface.
- **Rust Edition**: The project is configured to use the Rust 2021 edition in `Cargo.toml`.
- **Dependencies**:
    - The project uses the `clap` crate for command-line argument parsing.
    - The project uses the `statrs` crate for statistical calculations.
- **Testing**: Use `cargo test` to run all unit and integration tests.
