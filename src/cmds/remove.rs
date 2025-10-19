use crate::{
    cmds::tools::{is_command_installed, retrieve_commands},
    utils::{
        colors::COLORS,
        fs::delete_file,
        msgs::ask_for_confirmation,
        sys::{return_args, VARS},
    },
};

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

    if retrieve_commands("installed").is_empty() {
        return;
    }

    is_command_installed(name);

    ask_for_confirmation(&format!(
        "{red}Are you sure you want to delete command{yellow} \"{name}\"{red}?{reset}"
    ));

    delete_file(&format!(
        "{}/.local/share/cmdcreate/files/{name}",
        VARS.home
    ));
    delete_file(name);

    println!("\n{green}Removed command \"{name}\"");
}
