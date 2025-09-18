use crate::tools::utils::{is_command_installed, return_args, run_shell_command};

pub fn display() {
    let args = return_args();

    if args.len() < 2 {
        println!("Usage:\ncmdcreate display <command>");
        return;
    }

    if let Some(cmd) = args.get(1) {
        is_command_installed(cmd);
        run_shell_command(&format!("sudo echo -e \"Contents of command: {cmd}\n--------\"; sudo cat ~/.local/share/cmdcreate/files/{cmd}"),
                          || println!("Failed to read contents of command: {cmd}"));
    }
}
