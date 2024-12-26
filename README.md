# ChatGPT Conversation Scraper

A tool to scrape, extract, and save conversation data from ChatGPT-like web pages for later analysis or offline access. This project helps automate the process of extracting conversation content, including HTML and associated JSON data, from ChatGPT-powered applications.

## Features

- **Scrapes ChatGPT conversation data**: Extracts the conversation content and associated metadata.
- **Saves data locally**: Stores the conversation content in HTML format and JSON metadata.
- **Easy-to-use**: Just provide the URL, and the tool will fetch and save the data.

## Requirements

- **Rust**: The project is written in Rust. Ensure you have it installed on your machine.
- **Dependencies**:
    - `reqwest` for making HTTP requests.
    - `select` for parsing HTML and selecting elements.
    - `serde` and `serde_json` for JSON parsing.
    - `std::fs` for file handling.

## Installation

To get started, clone this repository to your local machine:

```bash
git clone https://github.com/yourusername/ScrapChat.git
cd ScrapChat
```

Next, build the project:

```bash
cargo build --release
```

## Usage

To use the scraper, simply run the following command:

```bash
cargo run -- <URL>
```

Where `<URL>` is the web page containing the ChatGPT conversation you want to scrape.

For example,

```shell
cargo run -- https://chatgpt.com/share/676d8989-e548-8004-8e13-4a16c689d4b6
```

### Example:

```bash
cargo run -- https://example.com/conversation
```

This will scrape the conversation, extract the necessary data, and save it in a local directory with the conversation's title as the folder name.

The data will be saved as:

- `linear_conversation.json` – The extracted JSON data that includes the conversation.

> NOTE: This is a work in progress -- more data will be extracted soon.

## Project Structure

```
chatgpt-conversation-scraper/
│
├── src/
│   └── main.rs            # Main application logic
│   └── ...                # Other utility files and contents
│
├── Cargo.toml             # Rust package manager file
└── README.md              # This file
```

## Contributions

Contributions are welcome! Please feel free to fork this repository and submit pull requests for any improvements, bug fixes, or new features.

To contribute:

1. Fork this repository.
2. Create a new branch.
3. Make your changes.
4. Submit a pull request.

> New feature requests are highly appreciated.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- This project uses the following libraries:
    - `reqwest`: HTTP client for fetching web pages.
    - `select`: HTML parsing and selection.
    - `serde` and `serde_json`: JSON parsing and serialization.
