use crate::utils::*;

pub fn remove() {
    let args = return_args();

    let blue = COLORS.blue;
    let yellow = COLORS.yellow;
    let red = COLORS.red;
    let green = COLORS.green;
    let reset = COLORS.reset;

    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}");
        return;
    }

    let name = args.get(1).unwrap();

    let exe = force_local_path(name);
    let exe_str = exe
        .to_str()
        .unwrap_or_else(|| panic!("{red}Error: Invalid UTF-8 in path.{reset}"));
    let installed_scripts: Vec<std::path::PathBuf> = retrieve_commands("installed");

    if installed_scripts.is_empty() {
        return;
    }

    is_command_installed(name);

    ask_for_confirmation(&format!(
        "{red}Are you sure you want to delete command{yellow} \"{name}\"{red}?{reset}"
    ));

    // Remove the script file
    run_shell_command(&format!("sudo rm -f {exe_str}"), || {
        error("Unable to retrieve remove command script.", "")
    });

    // Remove the symlink
    run_shell_command(&format!("sudo rm -f {name}"), || {
        println!("Error: Unable to retrieve remove command symlink.");
    });

    println!("\n{green}Removed command \"{name}\"");
}
