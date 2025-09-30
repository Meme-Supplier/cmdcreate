use crate::utils::*;

pub fn rename() {
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    let args = return_args();
    if args.len() < 3 {
        println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}");
        return;
    }
    let (name, new) = (args.get(1).unwrap(), args.get(2).unwrap());

    let installed_scripts = retrieve_commands("installed");
    if installed_scripts.is_empty() {
        return;
    }

    let mut count: i32 = 0;
    for script in installed_scripts {
        if script
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .contains(name)
        {
            count += 1;
        }
    }

    if count == 0 {
        error("Command doesn't exist: ", name);
        return;
    }

    run_shell_command(
        &format!("mv ~/.local/share/cmdcreate/files/{name} ~/.local/share/cmdcreate/files/{new}"),
        || error("Failed to rename command", name),
    );
    run_shell_command(&format!("sudo mv /usr/bin/{name} /usr/bin/{new}"), || {
        error("Failed to rename command", name)
    });
    run_shell_command(
        &format!("sudo ln -sf ~/.local/share/cmdcreate/files/{new} /usr/bin/{new}"),
        || {
            error("Failed to create command symlink", "");
        },
    );

    println!("{green}Created command {blue}\"{new}\"{reset}")
}
