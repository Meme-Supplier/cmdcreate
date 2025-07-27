use crate::tools::utils::{force_local_path, run_shell_command};
use std::path::Path;
use std::process;

pub fn create() {
    let args = crate::return_args();

    if let (Some(name_arg), Some(contents)) = (args.get(1), args.get(2)) {
        let original_path = Path::new(name_arg);

        let name = if original_path.starts_with("/usr/bin") {
            original_path.to_path_buf()
        } else if original_path.parent().is_some() || original_path.is_absolute() {
            // Starts with a path (like ./ or /something) -> use only the filename
            let filename = original_path
                .file_name()
                .expect("Invalid path: missing filename");
            Path::new("/usr/bin").join(filename)
        } else {
            // No path, just a bare name like "script" -> prepend /usr/bin
            Path::new("/usr/bin").join(name_arg)
        };

        let exe = force_local_path(&name);
        let exe_str = exe.to_str().expect("Invalid UTF-8 in path");

        // Make sure parent dir exists
        if let Some(parent) = exe.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create target dir");
        }

        // Write to a temp file first
        let pid = process::id();
        let temp_path = format!("/tmp/temp_cmd_file_{pid}.sh");
        std::fs::write(&temp_path, contents).expect("Failed to write temp file");

        // Use sudo to copy it to the final location
        run_shell_command(&format!("sudo cp {temp_path} {exe_str}"));
        run_shell_command(&format!("sudo chmod +x {exe_str}"));

        // Create symlink (optional if script is already in target path)
        run_shell_command(&format!("sudo ln -sf {exe_str} {}", name.to_string_lossy()));

        // print result summary
        println!("\nSuccess! Created executable:");
        println!("  Script path: {exe_str}");
        println!("  Symlink path: {}", name.to_string_lossy());
    } else {
        crate::display_usage();
    }
}
