# BDB Parser

## Overview

BDB Parser is a Rust-based library and binary crate designed to parse `.BDB` files, which contain a database of race tracks around the world. The library provides functionality to read the binary format of the `.BDB` file and store it into the internal defined data model. The internal data model implements serde serialization and deserialization. The binary crate offers a command-line tool for parsing `.BDB` files and displaying the output as a JSON string.

This parser was written as a complete black box reverse engineering exercise. It is not endorsed by the original author of the file format. The file was found on the VBOX Motorsports website, and seems to be used to store race track data for some of their hardware products. I found the file compressed into the `TRACKS.ZIP` file available on this page: https://www.vboxmotorsport.co.uk/index.php/us/customer-ct-track-database . Specifically, I used the "VBOX Touch" download.

## Features

- Parses `.BDB` files containing race track data.
- By default the binary tool outputs data in JSON format to stdout.

## File Structure

This file format was manually reverse engineered, there are some unknown fields in the file that cannot be parsed correctly.
For detailed information on the file structure, refer to the [BDB File Format Documentation](format.md).

## Getting Started

### Prerequisites

- Rust (edition 2021)
- Cargo

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/bdb_parser.git
    cd bdb_parser
    ```

2. Build the project:
    ```sh
    cargo build 
    ```

### Binary Usage

To run the parser, use the following command:
```sh
cargo run  <path_to_bdb_file>
```

Example:
```sh
cargo run -- TRACKS.BDB
```

### Binary Output

The output will be a JSON representation of the `.BDB` file content, printed to the console.

## Project Structure

- `src/bin/main.rs`: Entry point of the application. Handles argument parsing and file reading.
- `src/lib.rs`: Library entry point. Exports the main parsing functions and data structures.
- `src/models.rs`: Contains data structures representing the parsed data.
- `src/parsers.rs`: Contains functions to parse different sections of the `.BDB` file.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request, or if you have any insight on the unknown fields in the file format, please open an issue.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.