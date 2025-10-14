mod utils;
use utils::*;

mod cmds;
use cmds::*;

fn display_usage() {
    let (blue, cyan, yellow, magenta, reset) = (
        COLORS.blue,
        COLORS.cyan,
        COLORS.yellow,
        COLORS.magenta,
        COLORS.reset,
    );

    let lines: Vec<String> = vec![
        format!("cmdcreate {PROJ_VER}"),
        format!("Usage: cmdcreate {magenta}(flags){reset} [{blue}command{reset}, {cyan}argument{reset}] {yellow}<args> {magenta}(flags){reset}"),
        "\nCommands:".into(),
        format!("  {blue}create{yellow} <command> <contents>{reset}    Create a command"),
        format!("  {blue}remove {yellow}<command>{reset}               Remove a command"),
        format!("  {blue}edit   {yellow}<command>{reset}               Modify a command"),
        format!("  {blue}list{reset}                           Display installed commands"),
        format!("  {blue}search {yellow}<command>{reset}               Searches for matched command"),
        format!("  {blue}reset{reset}                          Removes all installed commands"),
        format!("  {blue}display {yellow}<command>{reset}              Display contents of a command"),
        format!("  {blue}rename {yellow}<command> <new name>{reset}    Renames a command"),
        "\n  Update:".into(),
        format!("    {blue}check{reset}                        Checks for updates"),
        format!("    {blue}update{reset}                       Updates your system"),
        "\nArguments:".into(),
        format!("  {cyan}-v{reset},{cyan} --version {reset}                 Displays version"),
        format!("  {cyan}-s{reset},{cyan} --supported_editors {reset}       Displays supported text editors"),
        format!("  {cyan}-c{reset},{cyan} --changelog {reset}               Displays changelog"),
        format!("  {cyan}-l{reset},{cyan} --license {reset}                 Displays license"),
        format!("  {cyan}-d{reset},{cyan} --debugging {reset}               Displays flags used for debugging"),
        "\n  Offline:".into(),
        format!("    {cyan}-g{reset},{cyan} --get_offline_files{reset}      Downloads files for offline use"),
        format!("    {cyan}-r{reset},{cyan} --remove_offline_files{reset}   Removes files for offline use"),
        "\nAbout:".into(),
        "   Cmdcreate allows you to create custom commands for your Linux Terminal".into(),
        "   without needing to enter the same \"complex\" commands over and over".into(),
        "   (unless if your are lazy like me).".into(),
        "\n   Cmdcreate will usually get smaller, more frequent updates than larger,".into(),
        "   less frequent updates. You should update Cmdcreate often to stay up to".into(),
        "   date.".into(),
    ];

    for line in lines {
        println!("{line}");
    }
}

fn main() {
    let (red, magenta, green, reset) = (COLORS.red, COLORS.magenta, COLORS.green, COLORS.reset);

    let args = utils::return_args();
    if args.is_empty() {
        display_usage();
        return;
    }

    if args_contains("--arg_count") | args_contains("-a") {
        println!(
            "{}DEBUG: ARG COUNT:{} {}",
            COLORS.magenta,
            COLORS.reset,
            args.len() - 1
        ); // Subtracts 1 to exclude debug flag
    }

    match args[0].as_str() {
        // Normal commands
        "create" => create::create(),
        "remove" => remove::remove(),
        "edit" => edit::edit(),
        "list" => list::list(),
        "search" => search::search(),
        "display" => display::display(),
        "rename" => rename::rename(),

        "reset" => {
            ask_for_confirmation(
                &format!("Are you sure you want remove ALL installed custom commands?\n{red}THIS CAN NOT BE UNDONE{reset}"),
            );
            ask_for_confirmation(&format!(
                "\n{}Are you SURE?{}\n(double check)",
                COLORS.red, COLORS.reset
            ));
            run_shell_command("rm -f ~/.local/share/cmdcreate/files/*", || {
                error("Unable to reset cmdcreate", "");
            });
            println!("\ncmdcreate has been reset.")
        }

        "check" => check_for_updates(),
        "update" => upgrader::upgrade(),

        // Arguments
        "--version" | "-v" => {
            println!("cmdcreate {PROJ_VER}");
        }
        "--supported_editors" | "-s" => {
            println!("Current supported editors:\n");
            for option in SUPPORTED_EDITORS {
                println!("{option}")
            }
        }

        "--get_offline_files" | "-g" => {
            println!("Downloading offline files...");

            run_shell_command(
                "
                mkdir -p ~/.local/share/cmdcreate/; \
                curl -o ~/.local/share/cmdcreate/LICENSE https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/LICENSE; \
                curl -o ~/.local/share/cmdcreate/changes.md https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/changes.md
                ", || {

                error(
                    "Failed to create directory",
                    "\"~/.local/share/cmdcreate/\"",
                );
            });

            println!("{green}Files downloaded successfully.{reset}");
        }

        "--remove_offline_files" | "-r" => {
            run_shell_command(
                "
                rm -f ~/.local/share/cmdcreate/changes.md \
                ~/.local/share/cmdcreate/LICENSE
                ",
                || error("Unable to remove files.", ""),
            );
            println!("Files removed successfully.");
        }

        "--license" | "-l" => {
            if args_contains("--offline") | args_contains("-o") {
                println!(
                    "{}",
                    read_file_to_string(&format!("{}/.local/share/cmdcreate/LICENSE", VARS.home))
                );

                return;
            }

            run_shell_command(
                "curl -s https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/LICENSE",
                || {
                    println!(
                        "{}",
                        read_file_to_string(&format!(
                            "{}/.local/share/cmdcreate/LICENSE",
                            VARS.home
                        ))
                    )
                },
            )
        }

        "--changelog" | "-c" => {
            if args_contains("--offline") | args_contains("-o") {
                println!(
                    "{}",
                    read_file_to_string(&format!(
                        "{}/.local/share/cmdcreate/changes.md",
                        VARS.home
                    ))
                    .trim()
                );
            } else {
                run_shell_command(
                    "curl -s https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/changes.md",
                    || {
                        println!(
                            "{}",
                            read_file_to_string(&format!(
                                "{}/.local/share/cmdcreate/changes.md",
                                VARS.home
                            ))
                            .trim()
                        )
                    }
                )
            }
        }

        "--debugging" | "-d" => {
            let lines: Vec<String> = vec![
                format!("Usage: cmdcreate {magenta}(flags){reset} [run] {magenta}(flags){reset}"),
                format!("  {magenta}-a{reset},{magenta} --arg_count{reset}             Displays number of arguments supplied"),
                format!("  {magenta}-F{reset},{magenta} --force_system_shell{reset}    Forces system shell to be used when running commands"),
                format!("  {magenta}-o{reset},{magenta} --offline{reset}               Run certain commands/arguments offline"),
                format!("  {magenta}-f{reset},{magenta} --force{reset}                 Skips confirmation for an action")
            ];

            for line in lines {
                println!("{line}")
            }
        }

        _ => {
            if args[0].starts_with("-") {
                error("Invalid argument:", &args[0]);
            }

            error("Invalid command:", &args[0])
        }
    }
}
