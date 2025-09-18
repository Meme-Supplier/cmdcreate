use crate::tools::utils::*;

pub fn remove() {
    let args = return_args();

    if args.len() < 2 {
        println!("Usage:\ncmdcreate remove <command>");
        return;
    }

    if let Some(name) = args.get(1) {
        let exe = force_local_path(name);
        let exe_str = exe.to_str().expect("Invalid UTF-8 in path");
        let installed_scripts: Vec<std::path::PathBuf> = retrieve_commands("installed");

        if installed_scripts.is_empty() {
            return;
        }

        is_command_installed(name);

        ask_for_confirmation(&format!(
            "Are you sure you want to delete command \"{name}\"?"
        ));

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
    }
}
