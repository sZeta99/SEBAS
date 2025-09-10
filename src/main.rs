use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod commands;
mod storage;
//NOTE 
//writecmd () { 
//  perl -e 'ioctl STDOUT, 0x5412, $_ for split //, do{ chomp($_ = <>); $_ }' ; 
//}

//# Example usage
//echo 'my test cmd' | writecmd


#[derive(Parser)]
#[command(name = "sebas")]
#[command(about = "Simply Elegant Bookmarking And Storing Commands")]
struct Cli {
    /// Optional custom storage location
    #[arg(short, long, global = true)]
    location: Option<PathBuf>,

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
        }) => {
            commands::add_command(
                command.clone(), 
                group.clone(), 
                comment.clone(), 
                aliases.clone(), 
                cli.location.clone()
            )
        },
        Some(Commands::Remove { identifier }) => {
            commands::remove_command(identifier, cli.location.clone())
        },
        Some(Commands::List) => {
            commands::list_commands(cli.location.clone())
        },
        Some(Commands::Edit { command }) => {
            commands::edit_command(command, cli.location.clone())
        },
        Some(Commands::Get { query }) => {
            commands::run_command(query, cli.location.clone())
        },
        None => {
            println!("No command specified. Use --help to see available commands.");
            Ok(())
        }
    }
}
