use crate::cmds::tools::retrieve_commands;
use crate::utils::{
    colors::COLORS,
    fs::{read_file_to_string, write_to_file},
    sys::{return_args, VARS},
};

pub fn export() {
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}export {yellow}<output directory>{reset}");
        return;
    }

    let output_path = args.get(1).unwrap();

    for script in retrieve_commands("installed") {
        let cmd = script.file_stem().unwrap_or_default().to_string_lossy();
        let cmd_contents =
            read_file_to_string(&format!("{}/.local/share/cmdcreate/files/{cmd}", VARS.home));
        write_to_file(
            &format!("{output_path}/export.cmdcreate"),
            &format!("{cmd}, {cmd_contents}"),
        );
    }

    println!("{green}Successfully exported installed commands to:{blue} \"{output_path}export.cmdcreate\"{green}.{reset}")
}
