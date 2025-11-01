use crate::utils::{
    colors::COLORS,
    fs::{read_file_to_string, write_to_file},
    sys::{return_args, run_shell_command, VARS},
};

pub fn import() {
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}");
        return;
    }

    let import_file = &args[1];
    let contents = read_file_to_string(import_file);

    for line in contents.lines() {
        if !line.contains(',') {
            continue;
        }

        let mut parts = line.splitn(2, ',');
        let name = parts.next().unwrap().trim();
        let data = parts.next().unwrap().trim();

        println!("{blue}Installing command: \"{green}{name}{reset}\"");

        let script = format!("{}/.local/share/cmdcreate/files/{}", VARS.home, name);
        write_to_file(&script, data);

        run_shell_command(&format!(
            "
            chmod +x {script}; \
            sudo ln -sf {script} /usr/bin/{name}
            ",
        ));
    }

    println!(
        "\n{green}Successfully imported commands from: {blue}\"{import_file}\"{green}.{reset}"
    );
}
