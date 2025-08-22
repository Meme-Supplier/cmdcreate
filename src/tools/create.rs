use crate::tools::utils::*;
use std::path::Path;
use std::process::id;

pub fn create() {
    let args = return_args();

    if args.len() < 3 {
        println!("Usage:\ncmdcreate create <command> <contents>");
        return
    }

    if let (Some(name_arg), Some(contents)) = (args.get(1), args.get(2)) {
        let original_path = Path::new(name_arg);

        let name = if original_path.starts_with("/usr/bin") {
            original_path.to_path_buf()
        } else if original_path.parent().is_some() || original_path.is_absolute() {
            let filename = original_path
                .file_name()
                .expect("Invalid path: missing filename");
            Path::new("/usr/bin").join(filename)
        } else {
            Path::new("/usr/bin").join(name_arg)
        };

        let exe = force_local_path(&name);
        let exe_str = exe.to_str().expect("Invalid UTF-8 in path");

        // Make sure parent dir exists
        if let Some(parent) = exe.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create target dir");
        }

        // Write to a temp file first
        let temp_path = format!("/tmp/temp_cmd_file_{}.sh", id());
        std::fs::write(&temp_path, contents).expect("Failed to write temp file");

        // Use sudo to copy it to the final location
        run_shell_command(&format!("sudo cp {temp_path} {exe_str}"), || {
            println!("Error: Failed to copy files.");
            return;
        });
        run_shell_command(&format!("sudo chmod +x {exe_str}"), || {
            println!("Error: Failed to mark command script as executable.");
            return;
        });

        // Create symlink (optional if script is already in target path)
        run_shell_command(
            &format!("sudo ln -sf {exe_str} {}", name.to_string_lossy()),
            || {
                println!("Error: Failed to create command symlink.");
                return;
            },
        );

        // print result summary
        let cmd = name.file_name().unwrap_or_default().to_string_lossy();

        println!("\nSuccess! Created command: {cmd}");
    } else {
        crate::display_usage();
    }
}
