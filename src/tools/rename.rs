use crate::tools::utils::*;

pub fn rename() {
    let args = return_args();
    let mut count: i32 = 0;

    if args.len() < 3 {
        println!("Usage:\ncmdcreate rename <command> <new name>");
        return;
    }

    if let (Some(name), Some(new)) = (args.get(1), args.get(2)) {
        let installed_scripts = retrieve_commands("installed");

        if installed_scripts.is_empty() {
            return;
        }

        for script in installed_scripts {
            let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();

            if file_stem.contains(name) {
                count += 1;
            }
        }

        if count == 0 {
            println!("Command: \"{name}\" doesn't exist");
            return;
        }

        run_shell_command(
            &format!(
                "mv ~/.local/share/cmdcreate/files/{name} ~/.local/share/cmdcreate/files/{new}"
            ),
            || error("Failed to rename command", &format!("{name}")),
        );
        run_shell_command(&format!("sudo mv /usr/bin/{name} /usr/bin/{new}"), || {
            error("Failed to rename command", &format!("{name}"))
        });
        run_shell_command(
            &format!("sudo ln -sf ~/.local/share/cmdcreate/files/{new} /usr/bin/{new}"),
            || {
                error("Failed to create command symlink", "");
                return;
            },
        );

        println!("Created command \"{new}\"")
    }
}
