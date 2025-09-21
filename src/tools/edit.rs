use crate::tools::utils::*;

pub fn edit() {
    let args = return_args();
    let install_dir = retrieve_commands("dir").get(0).cloned();

    if args.len() < 2 {
        println!("Usage:\ncmdcreate remove <command>");
        return;
    }

    let Some(install_dir) = install_dir else {
        println!("Install dir not found.");
        return;
    };

    let Some(editor) = SUPPORTED_EDITORS
        .iter()
        .find(|&&ed| is_editor_installed(ed))
    else {
        println!("No known editor found. Please install one like vim, nano, or gedit.");
        return;
    };

    let Some(file_name) = args.get(1) else {
        println!("No file name provided to edit.");
        return;
    };

    let file_path = install_dir.join(file_name);

    if !file_path.exists() {
        println!("File not found: {}", file_path.display());
        return;
    }

    run_shell_command(&format!("sudo {editor} {}", file_path.display()), || {
        error("Failed to edit your command", "");
        return;
    });
}
