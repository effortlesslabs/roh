Here's the README converted into Markdown format:

# OpenAI CLI

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
   cd openai-cli
   ```

3. **Build the Project**:
   ```bash
   cargo build --release
   ```
4. **(Optional) Install the CLI Globally**:
   ```bash
   cargo install --path .
   ```

## Usage

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
