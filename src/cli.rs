//! ---------------------------------------------------------------------------
//! File: cli.rs
//!
//! Description:
//!   Command-line interface (CLI) module for TermCord. Provides functions
//!   to display the program version, usage, and help information.
//!
//! Author: wh0crypt <wh0crypt@protonmail.com>
//! Created: 2025-10-26
//! Last Modified: 2025-10-26
//! Notes:
//!   This module is responsible for all user-facing CLI output.
//!
//! License:
//!   MIT License OR Apache License 2.0
//! ---------------------------------------------------------------------------

// SPDX-License-Identifier: MIT
// or use: SPDX-License-Identifier: Apache-2.0

/// Prints the current version of TermCord.
///
/// Reads the package version from the environment variable
/// `CARGO_PKG_VERSION` and prints it to the terminal.
pub fn print_version() {
    println!("TermCord version {}", env!("CARGO_PKG_VERSION"));
}

/// Prints a brief usage message.
///
/// Displays the general usage format and hints to use `help` for more
/// information about commands.
pub fn print_usage() {
    println!("Usage: termcord <command> [options]");
    println!("Try 'termcord help' to display the program information.");
}

/// Prints a detailed help message.
///
/// Shows information about TermCord, usage instructions, and available
/// commands. Useful for new users or when the user requests help.
pub fn print_help() {
    println!(
        "TermCord — A lightweight, IRC-style Discord client for your terminal, written in Rust.

Usage:
    termcord <command> [options]

Commands:
    help                 Show this message
    version              Show TermCord version
"
    );
}
