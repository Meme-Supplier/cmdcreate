use crate::utils::*;

pub fn create() {
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    let args = return_args();
    if args.len() < 3 {
        println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}");
        return;
    }
    let (name, contents) = (args.get(1).unwrap(), args.get(2).unwrap());

    let script = &format!("{}/.local/share/cmdcreate/files/{name}", VARS.home);

    let _ = std::fs::write(&script, contents);

    run_shell_command(
        &format!(
            "
            chmod +x {script}; \
            sudo ln -sf {script} /usr/bin/{name}
            ",
        ),
        || error("Failed to create command:", name),
    );

    println!("\n{green}Success! Created command: {blue}\"{name}\"{reset}",);
}
