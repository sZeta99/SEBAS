use std::path::PathBuf;

use clap::Subcommand;

use crate::commands::group::definition::GroupAction;

#[derive(Subcommand)]
pub enum Commands {
    /// Add a command to bookmarks
    #[command(alias = "a")]
    Add {
        /// Command to add (optional if using stdin)
        command: Option<String>,
        /// Group name (defaults to "Miscellaneous")
        #[arg(short, long)]
        group: Option<String>,
        /// Comment for the command
        #[arg(short, long)]
        comment: Option<String>,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// List saved commands
    #[command(alias = "ls")]
    List {
        /// Filter by group
        #[arg(short, long)]
        group: Option<String>,
        /// Show verbose output with full paths
        #[arg(short, long)]
        verbose: bool,
        /// Plain output for scripting
        #[arg(short, long)]
        plain: bool,
    },
    /// Edit a saved command
    #[command(alias = "e")]
    Edit {
        /// Command index or hash
        identifier: String,
        /// New command text
        #[arg(long)]
        new_command: Option<String>,
        /// New group name
        #[arg(long)]
        new_group: Option<String>,
        /// New comment
        #[arg(long)]
        new_comment: Option<String>,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// Remove a saved command
    #[command(alias = "rm")]
    Remove {
        /// Command index or hash
        identifier: String,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// Search commands by history
    #[command(alias = "h")]
    History {
        /// Search query
        query: Option<String>,
    },
    /// Obtain command
    #[command(alias = "o")]
    Obtain {
        /// Command index or hash
        identifier: Option<String>,
    },
    /// Group management
    #[command(alias = "g")]
    Group {
        #[command(subcommand)]
        action: GroupAction,
    },
    /// Initialize a .sebas folder
    Init {
        /// Path to initialize (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Sync commands from nested .sebas folders
    Sync,

}


