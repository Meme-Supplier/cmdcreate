use crate::{
    cmds::tools::retrieve_commands,
    utils::{
        colors::COLORS,
        msgs::error,
        sys::{return_args, run_shell_command},
    },
};
use std::process::Command;

pub const SUPPORTED_EDITORS: [&str; 13] = [
    "nvim",
    "vi",
    "vim",
    "nano",
    "micro",
    "code",
    "code-insiders",
    "gedit",
    "kate",
    "kwrite",
    "emacs",
    "vscodium",
    "vscodium-insiders",
];

pub fn is_editor_installed(editor: &str) -> bool {
    Command::new("which")
        .arg(editor)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn edit() {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}remove{yellow} <command>{reset}");
        return;
    }

    let binding = retrieve_commands("dir");
    let install_dir = binding.first();

    let Some(install_dir) = install_dir else {
        error("Installation directory not found.", "");
        return;
    };

    let Some(editor) = SUPPORTED_EDITORS
        .iter()
        .find(|&&ed| is_editor_installed(ed))
    else {
        error("No known editor is installed on your device.", "");
        return;
    };

    let file_path = install_dir.join(args.get(1).unwrap());

    if !file_path.exists() {
        error(
            "Command not installed:",
            &format!("{yellow}\"{}\"{reset}", &args.get(1).unwrap()),
        )
    }

    run_shell_command(&format!("sudo {editor} {}", file_path.display()), || {
        error("Failed to edit command:", args.get(1).unwrap())
    })
}
