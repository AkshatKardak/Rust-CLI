# Rust CLI Assistant

An AI-powered command-line assistant built with **Rust** and the **Google Gemini API**. Interact with Gemini directly from your terminal to solve coding problems, generate code, debug errors, and explore programming concepts across multiple languages.

## Features

- **AI-powered responses:** Generate code and explanations using the Gemini API.
- **Multi-language coding support:** Ask questions about Rust, Python, Java, C++, JavaScript, Go, and more.
- **Streaming responses:** Display generated text progressively in the terminal.
- **Interactive CLI:** Chat with the assistant through a simple terminal interface.
- **Conversation history:** Maintain context across messages within a session.
- **Model switching:** View or change the Gemini model using CLI commands.
- **Error handling:** Display useful messages for API and configuration errors.
- **Environment-based configuration:** Keep API credentials outside the source code.

## Tech Stack

| Technology | Purpose |
|---|---|
| Rust | Core application |
| Google Gemini API | AI-generated responses |
| Tokio | Asynchronous runtime |
| Reqwest | HTTP requests |
| Serde & Serde JSON | JSON serialization and deserialization |
| dotenvy | Environment variable configuration |
| Futures Util | Asynchronous response streaming |
| Anyhow | Error handling |

## Project Structure

```text
Rust-CLI/
├── src/
│   ├── main.rs       # Application entry point
│   ├── cli.rs        # Terminal interface and commands
│   ├── config.rs     # API key and model configuration
│   ├── gemini.rs     # Gemini API integration and streaming
│   └── chat.rs       # Conversation history and chat logic
├── .env              # Local environment variables
├── .env.example      # Example configuration
├── .gitignore
├── Cargo.toml        # Dependencies and package metadata
├── Cargo.lock        # Locked dependency versions
└── README.md
```

## Prerequisites

Before running the project, make sure you have:

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.
- A Google Gemini API key from [Google AI Studio](https://aistudio.google.com/apikey).
- An active internet connection.

Verify your Rust installation:

```bash
rustc --version
cargo --version
```

## Installation

**1. Clone the repository**

```bash
git clone https://github.com/AkshatKardak/Rust-CLI.git
cd Rust-CLI
```

**2. Configure environment variables**

Create a `.env` file in the project root.

```env
GEMINI_API_KEY=your_gemini_api_key
GEMINI_MODEL=gemini-3.8-flash
```

Replace `your_gemini_api_key` with your actual API key. The model shown is an example; use a model currently available to your Gemini API key.

**3. Install dependencies and build**

```bash
cargo build
```

**4. Run the application**

```bash
cargo run
```

## Usage

After launching the application, enter a prompt at the terminal:

```text
You > Write a Python function to find the second-largest distinct number in a list.

Gemini > [Generated solution and explanation]
```

The assistant supports coding tasks across multiple programming languages. You can request implementations, algorithm explanations, debugging assistance, and code improvements.

### Available Commands

| Command | Description |
|---|---|
| `/help` | Display available commands |
| `/model` | Show the currently configured model |
| `/model <name>` | Switch to another Gemini model |
| `/history` | View conversation history |
| `/clear` | Clear the current conversation history |
| `/exit` | Exit the application |
| `/quit` | Exit the application |

## Example Coding Prompts

Try these prompts after starting the CLI.

**1. Algorithm implementation**

```text
Implement an LRU Cache with get and put operations
in O(1) average time complexity.
```

**2. File processing**

```text
Write a program that finds duplicate files in a directory
using content hashes. Handle large files efficiently.
```

**3. Backend development**

```text
Build a REST API for an expense tracker with CRUD
operations, filtering, pagination, and database storage.
```

**4. Debugging**

```text
Analyze this code, identify the bug, explain why it occurs,
and provide a corrected implementation with test cases.
```

**5. Algorithm design**

```text
Build an expression evaluator that supports operator
precedence and parentheses without using eval().
```

## How It Works

1. The application loads the Gemini API key and model configuration from environment variables.
2. The CLI accepts a prompt from the user.
3. The chat module combines the new prompt with the current session history.
4. The Gemini client sends the conversation to the Gemini API.
5. The API response is streamed back and displayed in the terminal.
6. The user and assistant messages are stored in memory for subsequent requests.

## Security

- Never commit your `.env` file or expose your API key in source code.
- Keep `.env` listed in `.gitignore`.
- Do not share API keys in screenshots, logs, or public repositories.
- Rotate your API key if it is accidentally exposed.
- Review API usage limits and pricing before making frequent requests.

## Current Limitations

- An internet connection is required to communicate with Gemini.
- API availability, quotas, and usage costs depend on your Google AI Studio configuration.
- Conversation history is session-based and is not persisted after restarting the application unless persistence is implemented.
- Generated code should be reviewed and tested before use in production.
- Model availability depends on Google's current API offerings and your account access.

## Future Improvements

- [ ] Persistent conversation history using JSON or SQLite.
- [ ] Markdown-aware terminal rendering and syntax highlighting.
- [ ] Save and export AI-generated code to files.
- [ ] Retry logic for temporary API failures and rate limits.
- [ ] Configurable generation parameters.
- [ ] Dedicated coding modes for debugging, explanation, and code generation.
- [ ] Unit and integration tests.
- [ ] Command-line arguments for one-shot prompts and model selection.

## Contributing

Contributions, bug reports, and feature suggestions are welcome.

1. Fork the repository.
2. Create a feature branch.
3. Make and test your changes.
4. Submit a pull request describing your changes.

## License

No license has been specified yet. Add a `LICENSE` file before distributing the project under an open-source license.

---

**Built with Rust and Google Gemini.**
