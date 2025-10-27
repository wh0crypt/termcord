# TermCord

![Crates.io](https://img.shields.io/crates/v/termcord)
![GitHub Actions](https://github.com/wh0crypt/termcord/workflows/CI/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

A lightweight, IRC-style Discord client for your terminal, written in Rust.

## Description

TermCord provides a fast and minimalistic terminal-based interface to connect and interact with Discord servers.  
Users can read messages, send messages, and manage channels directly from the terminal.

> ⚠️ **Alpha:** The project is in active development. Features may change and bugs are expected.

## Status

**In development (Alpha)**

## Installation

### 1. From GitHub Releases

Download the precompiled binary for your OS from the [Releases](https://github.com/wh0crypt/termcord/releases) page.

**Linux / macOS:**

```bash
tar -xzf termcord-linux-<version>.tar.gz   # Linux
tar -xzf termcord-macos-<version>.tar.gz   # macOS
./termcord
```

**Windows:**

```powershell
Expand-Archive termcord-windows-<version>.zip
.\termcord.exe
```

### 2. From crates.io

If you have Rust and Cargo installed:

```bash
cargo install termcord
```

This will install the latest published version of TermCord and make the `termcord` executable available in your PATH.

## Usage

```bash
termcord help
termcord version
```

- `help` → Displays available commands and usage.  
- `version` → Shows the current TermCord version.

> More Discord-specific commands (connecting, sending messages, managing channels) will be added as the project develops.

## Contributing

Contributions are welcome!  

Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) before submitting any pull request.  
It contains guidelines on how to report issues, propose features, and submit code changes.

## License

Licensed under **MIT OR Apache-2.0**.  
See [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE) for details.

## Contact

- Author: wh0crypt  
- Email: [wh0crypt@protonmail.com](mailto:wh0crypt@protonmail.com)  
- GitHub: [https://github.com/wh0crypt/termcord](https://github.com/wh0crypt/termcord)
