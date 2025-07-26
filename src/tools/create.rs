use crate::tools::utils::{force_local_path, run_shell_command};

pub fn create() {
    let args = crate::return_args();

    if let (Some(name), Some(exe_arg), Some(contents)) = (args.get(1), args.get(2), args.get(3)) {
        if !name.starts_with("/usr/bin") && !name.starts_with("/usr/local/bin") {
            println!(
                "Value \"name ({name})\" should start with \"/usr/bin\" or \"/usr/local/bin\""
            );
            return;
        }

        let exe = force_local_path(exe_arg);
        let exe_str = exe.to_str().expect("Invalid UTF-8 in path");

        // Make sure parent dir exists
        if let Some(parent) = exe.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create target dir");
        }

        // Write to a temp file first
        let temp_path = "/tmp/temp_cmd_file.sh";
        std::fs::write(temp_path, contents).expect("Failed to write temp file");

        // Use sudo to copy it to the final location
        run_shell_command(&format!("sudo cp {temp_path} {exe_str}"));
        run_shell_command(&format!("sudo chmod +x {exe_str}"));

        // Create symlink
        run_shell_command(&format!("sudo ln -sf {exe_str} {name}"));

        // print result summary
        println!("Success! Created executable:");
        println!("  Script path: {exe_str}");
        println!("  Symlink path: {name}");
    } else {
        crate::display_usage();
    }
}
