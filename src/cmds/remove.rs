use crate::utils::*;

pub fn remove() {
    let (blue, yellow, red, green, reset) = (
        COLORS.blue,
        COLORS.yellow,
        COLORS.red,
        COLORS.green,
        COLORS.reset,
    );

    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}");
        return;
    }

    let name = args.get(1).unwrap();
    let script = format!("{}/.local/share/cmdcreate/files/{name}", VARS.home);

    if retrieve_commands("installed").is_empty() {
        return;
    }

    is_command_installed(name);

    ask_for_confirmation(&format!(
        "{red}Are you sure you want to delete command{yellow} \"{name}\"{red}?{reset}"
    ));

    run_shell_command(
        &format!(
            "
            sudo rm -f {script}; \
            sudo rm -f {name}
            "
        ),
        || error("Unable to retrieve remove command:", name),
    );

    println!("\n{green}Removed command \"{name}\"");
}
