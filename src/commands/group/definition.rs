use clap::Subcommand;

#[derive(Subcommand)]
pub enum GroupAction {
    /// List all groups
    #[command(alias = "lsg")]
    List,
    /// Add a new group
    #[command(alias = "addg")]
    Add {
        /// Group name
        name: String,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// Rename a group
    #[command(alias = "mvg")]
    Move {
        /// Old group name
        old_name: String,
        /// New group name
        new_name: String,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// Remove a group and all its commands
    #[command(alias = "rmg")]
    Remove {
        /// Group name
        name: String,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
}
