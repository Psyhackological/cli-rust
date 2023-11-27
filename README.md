## `grrs` - Text Search in Files Made Easy

`grrs` is a lightweight, efficient command-line tool designed to simplify the process of searching for specific text patterns within files. Pronounced "grass", this tool leverages the power of Rust programming to provide a fast and reliable search experience.

### Features
- **Fast and Efficient**: Built with Rust for optimal performance.
- **Easy to Use**: Simple command-line interface.
- **Flexible**: Search for any text pattern in a specified file.

## Installation

To install `grrs`, ensure you have Rust and Cargo installed on your system. Follow these steps:

1. Clone the repository: `git clone https://github.com/Psyhackological/grrs.git`
2. Navigate to the project directory: `cd grrs`
3. Build the project: `cargo build --release`
4. The executable can be found in `target/release/grrs`

## Usage

To search for a pattern in a file:

```
grrs <pattern> <path-to-file>
```

### Examples

- Search for "hello" in `file.txt`:
  ```
  grrs hello file.txt
  ```

## Testing

Run the following command to execute the tests:

```
cargo test
```

## Contributing

Contributions are welcome! Please read our contributing guidelines for details on how to submit pull requests, report issues, or suggest enhancements.

## License

This project is licensed under the `GPL-3.0-or-later`. See the LICENSE file for more details.

## Contact

For any queries or contributions, feel free to contact konradkon@duck.com.
