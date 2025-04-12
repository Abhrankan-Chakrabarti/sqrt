# Arbitrary-Precision Square Root Calculator

This is a Rust command-line utility that calculates the square root of a number to a user-specified number of decimal digits using fixed-point arithmetic. It uses the [Malachite](https://docs.rs/malachite/) library for fast and precise big integer operations.

## Features

- Computes the square root of any positive integer with high precision
- Supports both interactive mode and command-line arguments
- Clean fixed-point implementation
- Uses Malachite for efficient arithmetic

## Usage

You can run the binary in two ways:

### 1. Command-line Mode

```bash
cargo run --release -- [x] [digits]
```

### Example:

```bash
$ cargo run --release -- 2 50
√2 = 1.41421356237309504880168872420969807856967187537694...
```

This calculates √2 to 50 decimal digits.

### 2. Interactive Mode

If no arguments are given, the program will prompt you for input:

```bash
$ cargo run --release
Enter the number :    5
How many digits of √5? :    25
√5 = 2.2360679774997896964091736...
```

## Building

To build the project, make sure you have Rust installed. If not, install it via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone this repository and build it:

```bash
git clone https://github.com/Abhrankan-Chakrabarti/sqrt.git
cd sqrt
cargo build --release
```

Then run it:

```bash
cargo run --release
```

## License

This project is licensed under the MIT License.
