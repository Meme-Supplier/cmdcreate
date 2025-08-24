use crate::tools::utils::*;

pub fn remove() {
    let args = return_args();

    if args.len() < 2 {
        println!("Usage:\ncmdcreate remove <command>");
        return
    }

    if let Some(name) = args.get(1) {
        let exe = force_local_path(name);
        let exe_str = exe.to_str().expect("Invalid UTF-8 in path");
        let installed_scripts: Vec<std::path::PathBuf> = retrieve_commands("installed");
        
        if installed_scripts.is_empty() {
            return
        }

        for script in installed_scripts {
            let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();
            
            if file_stem.contains(name) {
                break;
            }

            println!("Error: Command \"{name}\" does not seem to be installed.");
            return
        }

        // Ask for confirmation
        println!("Are you sure you want to delete command \"{name}\"? (y/N)");
        let mut confirm = String::new();
        std::io::stdin().read_line(&mut confirm).unwrap();
        if confirm.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return;
        }

        // Remove the script file
        run_shell_command(&format!("sudo rm -f {exe_str}"), || {
            println!("Error: Unable to retrieve remove command script file.");
            return;
        });

        // Remove the symlink
        run_shell_command(&format!("sudo rm -f {name}"), || {
            println!("Error: Unable to retrieve remove command symlink.");
            return;
        });

        println!("\nRemoved command \"{name}\"");
    } else {
        crate::display_usage();
    }
}
