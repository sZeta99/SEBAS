use anyhow::{Context, Error};
use colored::Colorize;
use termion::{event::Key, input::TermRead};
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use crossterm::event;
use std::io::{self, stdout, Write};

use crate::commands::core::definition::ResolvedCommand;

pub fn sebas_fzf_run(commands: Vec<ResolvedCommand>) -> Result<(),Error> {

    
 // Create a vector of display strings and corresponding commands
    let mut display_items = Vec::new();
    let mut command_map = Vec::new();
    let mut index = 0;
    // Build the display list and command mapping
    for command in &commands {
        index += 1;
        let display = format!("{:2}. {}", index + 1, command.command.command);   
        display_items.push(display);
        command_map.push(Some(command.clone()));
    }
   

    // Setup terminal
     let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

    let stdin = io::stdin();
    let mut keys = stdin.keys();
    
    let mut selected_index = 1; // Start with the first actual command
    
    // Find the first actual command if we're not already on one
    while selected_index < command_map.len() && command_map[selected_index].is_none() {
        selected_index += 1;
    }

    loop {
        // Clear screen and reset cursor
        write!(screen, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
        writeln!(screen, "{}\r{}", "Stored Commands:".bold().blue(),"===============".blue())?;
        
        // Display items with highlighting for the selected one
        for (i, item) in display_items.iter().enumerate() {
            if i == selected_index {
                writeln!(screen, "{} <<<", item.bold().on_blue())?;
            } else {
                writeln!(screen, "{}", item)?;
            }
        }
        
        // Instructions at the bottom
        writeln!(screen, "{}{}\r{}", commands.len().to_string().bold().blue(), " Total Command." ,"Use ↑/↓ keys to navigate, Enter to execute, q to quit".yellow()).context("Failed to Render")?;
        screen.flush()?;
        
        // Get user key input
        if let Some(Ok(key)) = keys.next() {
            match key {
                Key::Char('q') | Key::Ctrl('c') | Key::Esc => {
                    break;
                },
                Key::Char('\n') => {
                    // Execute selected command if it exists
                    if let Some(cmd_entry) = &command_map[selected_index] {
                        // Return to normal terminal mode temporarily
                        drop(screen);
                        //drop(stdout);
                        
                        println!("\nExecuting: {}", cmd_entry.command.command.green());
                        
                        // Execute the command
                        //match execute_command(&cmd_entry.command) {
                        //    Ok(_) => println!("Command executed successfully."),
                        //    Err(e) => println!("Error executing command: {}", e),
                        //}
                        
                        println!("Press Enter to continue...");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        
                        // Return to raw mode
                        //screen = stdout()
                        //       .into_raw_mode()
                        //       .unwrap()
                        //       .into_alternate_screen()
                        //      .unwrap();
                        
                    }
                    break;
                },
                Key::Down | Key::Char('j') => {
                    // Move down
                    selected_index = find_next_command(selected_index, &command_map, true);
                },
                Key::Up | Key::Char('k') => {
                    // Move up
                    selected_index = find_next_command(selected_index, &command_map, false);
                },
                _ => {}
            }
        }
    }

    Ok(())
}
// Helper function to find the next or previous command
fn find_next_command(current: usize, command_map: &[Option<ResolvedCommand>], forward: bool) -> usize {
    let len = command_map.len();
    
    if forward {
        // Find next command
        for i in 1..len {
            let idx = (current + i) % len;
            if command_map[idx].is_some() {
                return idx;
            }
        }
    } else {
        // Find previous command
        for i in 1..len {
            let idx = (current + len - i) % len;
            if command_map[idx].is_some() {
                return idx;
            }
        }
    }
    
    current // Return current if no other command found
}





