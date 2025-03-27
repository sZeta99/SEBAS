use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod commands;
mod storage;

#[derive(Parser)]
#[command(name = "sebas")]
#[command(about = "Simply Elegant Bookmarking And Storing Commands")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new command
    #[command(alias = "a")]
    Add {
        /// The command to store
        command: String,

        /// Optional group/category
        #[arg(short, long)]
        group: Option<String>,

        /// Optional comment
        #[arg(short, long)]
        comment: Option<String>,

        /// Optional aliases (comma-separated)
        #[arg(short, long)]
        aliases: Option<String>,

        /// Optional custom storage location
        #[arg(short, long)]
        location: Option<PathBuf>,
    },

    /// Remove a command
    #[command(alias = "r")]
    Remove {
        /// Alias or command to remove
        identifier: String,
    },

    /// List all stored commands
    #[command(alias = "l")]
    List,

    /// Search for commands
    #[command(alias = "s")]
    Search {
        /// Search term
        query: String,
    },

    /// Edit an existing command
    #[command(alias = "e")]
    Edit {
        /// Command to edit
        command: String,
    },

    /// Retrieve a command
    #[command(alias = "g")]
    Get {
        /// Search term to find the command
        query: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { 
            command, 
            group, 
            comment, 
            aliases, 
            location 
        }) => {
            commands::add_command(
                command.clone(), 
                group.clone(), 
                comment.clone(), 
                aliases.clone(), 
                location.clone()
            )
        },
        Some(Commands::Remove { identifier }) => {
            commands::remove_command(identifier)
        },
        Some(Commands::List) => {
            commands::list_commands()
        },
        Some(Commands::Search { query }) => {
            commands::search_commands(query)
        },
        Some(Commands::Edit { command }) => {
            commands::edit_command(command)
        },
        Some(Commands::Get { query }) => {
            commands::run_command(query)
        },
        None => {
            println!("No command specified. Use --help to see available commands.");
            Ok(())
        }
    }
}
