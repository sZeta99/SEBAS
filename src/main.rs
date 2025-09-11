
//NOTE 
//writecmd () { 
//  perl -e 'ioctl STDOUT, 0x5412, $_ for split //, do{ chomp($_ = <>); $_ }' ; 
//}

//# Example usage
//echo 'my test cmd' | writecmd

/*
*
*_sebas_fzf_run() {

  # sebas ls --plain prints each saved command on its own line,

  # optionally prefixed by "[1]" or "c1f2". Adjust to your actual output.

  local cmd

  cmd=$(sebas ls --plain | fzf --height 40% --border \

        --prompt="SEBAS› " \

        --preview 'echo {}' \

        --bind 'enter:accept')

  if [[ -n "$cmd" ]]; then

    # If `ls` prepends an index, strip it; e.g. "[1] git status" → "git status"

    cmd=${cmd#*\] }  

    eval "$cmd"

  fi

}

# Create a short alias for it:

alias sr='_sebas_fzf_run'
*
* */
mod utils;
mod commands;
use clap::{Parser};
use std::
    path::PathBuf
;

use crate::{commands::{commands::definition::Commands, group::definition::GroupAction}, utils::fzf::sebas_fzf_run};

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
        Commands::Completions { shell } => {
            println!("Shell completions for {} not yet implemented", shell);
            println!("This would generate completion scripts for your shell");
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
                    let _ = sebas_fzf_run();
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
