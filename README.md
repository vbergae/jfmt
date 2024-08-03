# jfmt - JSON Formatter

jfmt is a command-line application for parsing and formatting JSON text. It reads JSON input from the terminal, formats it for readability, and outputs the formatted JSON to the terminal.

## Features

- Parse and format JSON text
- Output formatted JSON to the terminal
- Simple and easy-to-use command-line interface

## Installation

To build and install `jfmt`, you need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/jfmt.git
    cd jfmt
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Install the application:
    ```sh
    cargo install --path .
    ```

## Usage

After installation, you can use `jfmt` from the terminal:

```sh
jfmt < input.json
```

Alternatively, you can pipe JSON data to `jfmt`:

```sh
echo '{"name":"John","age":30,"city":"New York"}' | jfmt
```

## Examples

### Example 1: Formatting JSON from a file

```sh
jfmt < data.json
```

### Example 2: Formatting JSON from a string

```sh
echo '{"name":"John","age":30,"city":"New York"}' | jfmt
```

## Development

To contribute to `jfmt`, follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them.
4. Push your changes to your fork.
5. Create a pull request to the main repository.

### Running Tests

To run the tests for `jfmt`, use the following command:

```sh
cargo test
```

## License

This project is licensed under the GPL v3 License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or feedback, please open an issue on the [GitHub repository](https://github.com/yourusername/jfmt) or contact the project maintainer.

---

Happy formatting!
