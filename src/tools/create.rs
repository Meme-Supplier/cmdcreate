use crate::tools::utils::*;
use std::path::Path;
use std::process::id;

pub fn create() {
    let args = return_args();

    if args.len() < 3 {
        println!("Usage:\ncmdcreate create <command> <contents>");
        return;
    }

    let (name_arg, contents) = (args.get(1).unwrap(), args.get(2).unwrap());

    let original_path = Path::new(name_arg);

    let name = if original_path.starts_with("/usr/bin") {
        original_path.to_path_buf()
    } else if original_path.parent().is_some() || original_path.is_absolute() {
        Path::new("/usr/bin").join(
            original_path
                .file_name()
                .expect("Invalid path: missing filename"),
        )
    } else {
        Path::new("/usr/bin").join(name_arg)
    };

    let exe = force_local_path(&name);
    let exe_str = exe.to_str().expect("Invalid UTF-8 in path");

    if let Some(parent) = exe.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create target dir");
    }

    // Write to a temp file first
    let temp_path = format!("/tmp/temp_cmd_file_{}.sh", id());
    std::fs::write(&temp_path, contents).expect("Failed to write temp file");

    run_shell_command(&format!("sudo cp {temp_path} {exe_str}"), || {
        error("Failed to copy files.", "");
    });
    run_shell_command(&format!("sudo chmod +x {exe_str}"), || {
        error("Failed to mark command script as executable.", "");
    });

    run_shell_command(
        &format!("sudo ln -sf {exe_str} {}", name.to_string_lossy()),
        || {
            error("Failed to create command symlink.", "");
        },
    );

    println!(
        "\nSuccess! Created command: {}",
        name.file_name().unwrap_or_default().to_string_lossy()
    );
}
