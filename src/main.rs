/************************************************
* File: main.rs
* Author: Michal Švrček
*
* DevDock CLI entry point and session management
*
* ver. 0.3.0
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
    /// Preview a session without launching applications
    #[arg(long, global = true)]
    dry_run: bool,

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

    /// Launch a project session
    Open { name: String },

    /// Show session configuration
    Info { name: String },

    /// Edit session configuration
    Edit { name: String },

    /// Remove a project session
    Remove {
        name: String,

        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },

    /// Launch a session directly by its name
    #[command(external_subcommand)]
    Session(Vec<String>),
}

fn open_session(name: &str, dry_run: bool) -> Result<(), String> {
    if dry_run {
        println!("[DRY RUN] No applications will be launched.\n");
        return config::info(name);
    }

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

        Some(Commands::Open { name }) => open_session(&name, cli.dry_run),

        Some(Commands::Info { name }) => config::info(&name),

        Some(Commands::Edit { name }) => config::edit(&name),

        Some(Commands::Remove { name, yes }) => config::remove(&name, yes),

        Some(Commands::Session(args)) => {
            let Some(name) = args.first() else {
                return Err("Usage: dock <session> [--dry-run]".into());
            };

            let mut dry_run = cli.dry_run;

            for arg in args.iter().skip(1) {
                match arg.as_str() {
                    "--dry-run" => dry_run = true,
                    _ => {
                        return Err(format!("Unknown session argument: {arg}"));
                    }
                }
            }

            open_session(name, dry_run)
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
