mod utils;
mod commands;
use clap::{Parser};
use std::
    path::PathBuf
;

use crate::{commands::{commands::definition::Commands, group::definition::GroupAction}, utils::fzf::sebas_fzf_run, };

#[derive(Parser)]
#[command(name = "sebas")]
#[command(about = "Simply Elegant Bookmarked Alternatives for commandS")]
#[command(version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

struct SebasApp {
    sebas_dir: PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            SebasApp::init_folder(path)?;
        }
        Commands::Sync => {
            SebasApp::sync_folders()?;
        }

        _ => {
            let app = SebasApp::new()?;
            
            match cli.command {
                Commands::Add { command, group, comment, yes } => {
                    app.add_command(command, group, comment, yes)?;
                }
                Commands::List { group, verbose, plain } => {
                    app.list_commands(group, verbose, plain)?;
                }
                Commands::Edit { identifier, new_command, new_group, new_comment, yes } => {
                    app.edit_command(&identifier, new_command, new_group, new_comment, yes)?;
                }
                Commands::Remove { identifier, yes } => {
                    app.remove_command(&identifier, yes)?;
                }
                Commands::Search { query } => {
                    //app.search_commands(&query)?;
                    let _ = sebas_fzf_run(app.resolve_all_commands());
                }
                Commands::Get { identifier } => {
                    app.get_command(&identifier)?;
                }
                Commands::Group { action } => {
                    match action {
                        GroupAction::List => app.list_groups()?,
                        GroupAction::Add { name, yes } => app.add_group(&name, yes)?,
                        GroupAction::Move { old_name, new_name, yes } => app.rename_group(&old_name, &new_name, yes)?,
                        GroupAction::Remove { name, yes } => app.remove_group(&name, yes)?,
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
