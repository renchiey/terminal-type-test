# Terminal Type Test

A terminal-based typing test built with Rust and [ratatui](https://github.com/ratatui-org/ratatui). Inspired by [monkeytype](https://monkeytype.com/).

![Terminal Type Test](https://img.shields.io/badge/built_with-Rust-orange)

## Features

- 500 random words per session
- Real-time color feedback: correct (white), incorrect (red), untyped (gray)
- Block cursor tracks current position
- Delete char or whole word with keyboard shortcuts

## Controls

| Key | Action |
|-----|--------|
| Type | Enter character |
| `Space` | Advance to next word |
| `Backspace` | Delete last character (goes back a word if current is empty) |
| `Alt+Backspace` | Delete entire current word |
| `Esc` | Quit |

## Installation

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/renchiey/terminal_type_test
cd terminal_type_test
cargo run --release
```

## Tech Stack

- [`ratatui`](https://github.com/ratatui-org/ratatui) — TUI framework
- [`crossterm`](https://github.com/crossterm-rs/crossterm) — terminal input/output
- [`rand`](https://github.com/rust-random/rand) — word shuffle
