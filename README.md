# TODOR

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

A terminal-based todo application built with Rust and Ratatui.

## SCRENNSHOTS

![01](./assets/01.png)
![02](./assets/02.png)

## Installation

```bash
git clone <repository-url>
cd todor
cargo install --path .
```

This installs the `todor` binary to `~/.cargo/bin/`. Make sure `~/.cargo/bin` is in your PATH:

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$HOME/.cargo/bin:$PATH"
```

## Data

Todos are automatically saved to `./config/todor/todos.json`.
