/// Module for handling command renaming functionality in cmdcreate
///
/// This module provides functionality to rename existing commands while maintaining
/// their functionality and system integration. It handles both the command file
/// renaming and updating system symlinks.
use crate::{
    cmds::tools::retrieve_commands, // Command management utilities
    utils::{
        colors::COLORS,                        // Terminal color formatting
        msgs::error,                           // Error message handling
        sys::{return_args, run_shell_command}, // System operations
    },
};

/// Renames an existing command to a new name
///
/// This function:
/// 1. Validates command existence
/// 2. Renames the command file
/// 3. Updates system symlinks
/// 4. Maintains command functionality
///
/// # Usage
/// ```bash
/// cmdcreate rename <old_name> <new_name>
/// ```
pub fn rename() {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    // Get command line arguments and validate argument count
    let args = return_args();
    if args.len() < 3 {
        println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}");
        return;
    }

    // Extract old and new command names
    let (name, new) = (args.get(1).unwrap(), args.get(2).unwrap());

    // Get list of installed commands and validate there are commands to rename
    let installed_scripts = retrieve_commands("installed");
    if installed_scripts.is_empty() {
        return;
    }

    // Check if the command exists by counting matching scripts
    let mut count: i32 = 0;
    for script in installed_scripts {
        if script
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .contains(name)
        {
            count += 1
        }
    }

    // Error if command doesn't exist
    if count == 0 {
        error("Command doesn't exist:", name);
    }

    // Perform the rename operation:
    // 1. Rename the command file in cmdcreate's directory
    // 2. Move the system symlink to the new name
    // 3. Update the symlink to point to the new file
    run_shell_command(&format!(
        "
            mv ~/.local/share/cmdcreate/files/{name} ~/.local/share/cmdcreate/files/{new}; \
            sudo mv /usr/bin/{name} /usr/bin/{new}; \
            sudo ln -sf ~/.local/share/cmdcreate/files/{new} /usr/bin/{new}; \
            "
    ));

    // Confirm successful rename to user
    println!("{green}Created command {blue}\"{new}\"{reset}")
}
