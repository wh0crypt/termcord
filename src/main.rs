//! ---------------------------------------------------------------------------
//! File: main.rs
//!
//! Description:
//!   Entry point of the application. Initializes and runs the program,
//!   manages configuration, and starts the main logic.
//!
//! Author: wh0crypt <wh0crypt@protonmail.com>
//! Created: 2025-10-26
//! Last Modified: 2025-10-26
//! Notes:
//!   This file contains the main function and orchestrates modules of the app.
//!
//! License:
//!   MIT License OR Apache License 2.0
//! ---------------------------------------------------------------------------

// SPDX-License-Identifier: MIT
// or use: SPDX-License-Identifier: Apache-2.0

mod cli;

use std::env;

/// Entry point of the TermCord application.
///
/// This function initializes the application, parses command-line arguments,
/// and starts the main program logic. All setup tasks such as configuration
/// loading and logging should be performed here.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cli::print_usage();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "version" | "-v" | "--version" => cli::print_version(),
        "help" | "-h" | "--help" => cli::print_help(),
        _ => {
            println!("Error: Unknown command '{}'.\n", command);
            cli::print_usage();
        }
    }
}
