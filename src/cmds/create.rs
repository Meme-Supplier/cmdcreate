use crate::utils::*;

use std::path::Path;
use std::process::id;

pub fn create() {
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    let args = return_args();
    if args.len() < 3 {
        println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}");
        return;
    }
    let (name_arg, contents) = (args.get(1).unwrap(), args.get(2).unwrap());

    let original_path = Path::new(name_arg);

    let name = if original_path.starts_with("/usr/bin") {
        original_path.to_path_buf()
    } else if original_path.parent().is_some() || original_path.is_absolute() {
        Path::new("/usr/bin").join(original_path.file_name().unwrap_or_else(|| {
            panic!(
                "{}Error: Invalid path: Missing filename{}",
                COLORS.red, COLORS.reset
            )
        }))
    } else {
        Path::new("/usr/bin").join(name_arg)
    };

    let exe = force_local_path(&name);
    let exe_str = exe
        .to_str()
        .unwrap_or_else(|| panic!("{}Error: Invalid UTF-8 in path{}", COLORS.red, COLORS.reset));

    if let Some(parent) = exe.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // Write to a temp file first
    let temp_path = format!("/tmp/temp_cmd_file_{}.sh", id());
    let _ = std::fs::write(&temp_path, contents);

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
        "\n{green}Success! Created command: {blue}\"{}\"{reset}",
        name.file_name().unwrap_or_default().to_string_lossy()
    );
}
