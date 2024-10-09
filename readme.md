# Roh CLI

A simple command-line interface (CLI) for interacting with OpenAI's API to fetch completions and generate responses in Markdown format. Built using Rust.

## Features

- Ask questions to OpenAI and receive responses formatted in Markdown.
- Configuration management for API key and base URI.
- User-friendly command structure.

## Installation

1. **Install Rust**: Make sure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs/).
2. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/openai-cli.git
   cd roh-cli
   ```

## Usage

To build and use your Rust CLI in the terminal, follow these steps:

1. **Build Your CLI**:
   Navigate to your Rust project directory in the terminal and run:

   ```bash
   cargo build --release
   ```

   This command compiles your CLI and places the executable in the `target/release` directory.

2. **Run Your CLI**:
   You can run the CLI directly from the terminal using:

   ```bash
   ./target/release/roh
   ```

3. **Add to PATH** (optional):
   If you want to use your CLI globally without specifying the path, you can add the `target/release` directory to your `PATH` environment variable or move the binary to a directory that is already in your `PATH`, like `/usr/local/bin`:

   ```bash
   mv ./target/release/roh /usr/local/bin/
   ```

4. **Make It Executable** (if necessary):
   If you're on Unix-like systems, you might need to make the binary executable:
   ```bash
   chmod +x /usr/local/bin/roh
   ```

Now you should be able to run your CLI from anywhere in the terminal by just typing `roh`.

### Configuration

Before using the CLI, you need to configure it with your OpenAI API key and the base URI. Run the following command:

```bash
roh config <API_KEY> <BASE_URI>
```

### Asking Questions

To ask a question, use the `ask` command followed by your question:

```bash
roh ask "What is the capital of France?"
```

### Viewing Configuration

To view your current configuration, use the `get-config` command:

```bash
roh get-config
```

### Example

```bash
roh ask "Explain the theory of relativity."
```

## Error Handling

If there are any errors during the request, an error message will be displayed in the terminal.

## Contributing

Feel free to open issues or submit pull requests for any features or fixes.

## License

This project is licensed under the MIT License.
