/************************************************
* File: main.rs
* Author: Michal Švrček
*
* DevDock CLI entry point and command handling
*
* ver. 0.2.0
*************************************************/

mod config;
mod launcher;

use clap::{Parser, Subcommand};
use std::{env, path::PathBuf, process};

#[derive(Parser)]
#[command(name = "dock")]
#[command(version)]
#[command(about = "DevDock - Development workspace manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List available project sessions
    List,

    /// Create a new project session
    Add {
        name: String,

        #[arg(long)]
        path: Option<PathBuf>,
    },

    /// Open a project session
    Open { name: String },

    /// Edit a project session configuration
    Edit { name: String },

    /// Open a project session by its name
    #[command(external_subcommand)]
    Session(Vec<String>),
}

fn open_session(name: &str) -> Result<(), String> {
    let session = config::load(name)?;
    launcher::launch(&session)
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::List) => config::list(),

        Some(Commands::Add { name, path }) => {
            let folder = match path {
                Some(path) => path,
                None => env::current_dir().map_err(|e| e.to_string())?,
            };

            config::add(&name, &folder)
        }

        Some(Commands::Open { name }) => open_session(&name),

        Some(Commands::Edit { name }) => config::edit(&name),

        Some(Commands::Session(args)) => {
            if args.len() != 1 {
                return Err("Usage: dock <session>".into());
            }

            open_session(&args[0])
        }

        None => {
            println!("DevDock - Development workspace manager");
            println!("Use --help for available commands.");
            Ok(())
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}
