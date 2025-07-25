use crate::tools::utils::{force_local_path, run_shell_command};

pub fn remove() {
    let args = crate::return_args();

    if let (Some(name), Some(exe_arg)) = (args.get(1), args.get(2)) {
        let exe = force_local_path(exe_arg);
        let exe_str = exe.to_str().expect("Invalid UTF-8 in path");

        // Remove the script file
        run_shell_command(&format!("sudo rm -f {exe_str}"));

        // Remove the symlink
        run_shell_command(&format!("sudo rm -f {name}"));

        println!("Removed {exe_str} and symlink {name}");
    } else {
        crate::display_usage();
    }
}
