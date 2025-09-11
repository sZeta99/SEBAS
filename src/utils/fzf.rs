use colored::Colorize;
use termion::{event::Key, input::TermRead};
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use crossterm::event;
use std::io::{self, stdout, Write};

pub fn sebas_fzf_run() -> Result<(), String> {
    // Step 1: Run `sebas ls --plain` to get the list of commands
    let sebas_output = std::process::Command::new("sebas")
        .arg("ls")
        .arg("--plain")
        .output()
        .map_err(|e| format!("Failed to run 'sebas ls --plain': {}", e))?;

    if !sebas_output.status.success() {
        return Err(format!(
            "Command 'sebas ls --plain' failed with: {}",
            String::from_utf8_lossy(&sebas_output.stderr)
        ));
    }

    let commands = String::from_utf8(sebas_output.stdout)
        .map_err(|e| format!("Failed to parse 'sebas ls --plain' output: {}", e))?;

    let total_commands: Vec<String> = commands.lines().map(|s| s.to_string()).collect();
 // Create a vector of display strings and corresponding commands
    let mut display_items = Vec::new();
    let mut command_map = Vec::new();
    let mut index = 0;
    // Build the display list and command mapping
    for command in &total_commands {
        index += 1;
        let mut display = format!("{:2}. {}", index + 1, command);   
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
        writeln!(screen, "{}{}\r{}", total_commands.to_string().bold().blue(), " Total Command." ,"Use ↑/↓ keys to navigate, Enter to execute, q to quit".yellow())?;
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
                        
                        println!("\nExecuting: {}", cmd_entry.command.green());
                        
                        // Execute the command
                        match execute_command(&cmd_entry.command) {
                            Ok(_) => println!("Command executed successfully."),
                            Err(e) => println!("Error executing command: {}", e),
                        }
                        
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





