# Rust Endpoint URL Processor

This Rust project processes endpoint URLs by identifying and replacing various types of IDs with placeholders. The types of IDs processed include UUIDs, numeric IDs, and alphanumeric IDs. The project ensures that single IDs are replaced with `__ID__` and multiple IDs are replaced with `__IDs__`.

## Features

- **UUID Detection**: Identifies and replaces standard UUIDs.
- **Numeric ID Detection**: Identifies and replaces numeric values with three or more digits.
- **Alphanumeric ID Detection**: Identifies and replaces alphanumeric strings with seven or more characters containing both letters and digits.
- **Replacement Logic**: Replaces single IDs with `__ID__` and multiple IDs with `__IDs__`.

## Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/username/repository.git



 **Build the project**:
   ```bash
   cargo build
   ```

**Run the server**:
   ```bash
   cargo run
   ```


##Video:
https://github.com/sarthak20574/rust_assigment2-nebuliaq-placement-iiitd-SarthakDixit-2024
