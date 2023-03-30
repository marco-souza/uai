# OpenAI CLI

This package is a simple CLI tool to comunicate with OpenAI API, to be used as a CLI personal assistent.

## Usage

1. Clone the project
2. Add a `OPENAI_API_KEY` to your environment

### Install

1. Install it with `make install`
2. Done! Call `oai how are you doing\?`

### Development

1. Call `cargo run "how are you doing?"`

## Roadmap

- [ ] if no args was passed, open a conversational chat
- [x] keep context logs to make conversations more fluid
- [ ] make it pluggable (allowing to pass files with the role description)
- [ ] add tests
