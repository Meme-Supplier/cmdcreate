use crate::tools::utils::{force_local_path, run_shell_command};

pub fn remove() {
    let args = crate::return_args();

    if let Some(name) = args.get(1) {
        let exe = force_local_path(name);
        let exe_str = exe.to_str().expect("Invalid UTF-8 in path");

        // Ask for confirmation
        println!("Are you sure you want to delete command \"{name}\"? (y/N)");
        let mut confirm = String::new();
        std::io::stdin().read_line(&mut confirm).unwrap();
        if confirm.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return;
        }

        // Remove the script file
        run_shell_command(&format!("sudo rm -f {exe_str}"));

        // Remove the symlink
        run_shell_command(&format!("sudo rm -f {name}"));

        println!("\nRemoved command \"{name}\"");
    } else {
        crate::display_usage();
    }
}
