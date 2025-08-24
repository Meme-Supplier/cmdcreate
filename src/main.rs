mod tools;
use tools::*;

use crate::tools::utils::{args_contains, run_shell_command};

static PROJ_VER: &str = "v0.5.2";

fn display_usage() {
    let lines: [&str; 25] = [
        &format!("cmdcreate {PROJ_VER}"),
        "\nCommands:",
        "  create <command> <contents>    Create a custom command",
        "  remove <command>               Remove a custom command",
        "  edit   <command>               Modify a custom command",
        "  list                           Display installed commands",
        "  search <command>               Searches for matched command",
        "  reset                          Removes all installed commands",
        "\nArguments:",
        "  --credits                      Displays credits for cmdcreate",
        "  --version                      Displays version",
        "  --supported_editors            Displays supported text editors",
        "  --changelog                    Displays changelog",
        "  --license                      Displays license",
        "  --debugging                    Displays flags used for debugging",
        "\nOffline:",
        "  --get_offline_files            Downloads files for offline use",
        "  --remove_offline_files         Removes files for offline use",
        "\nAbout:",
        "   Cmdcreate allows you to create custom commands for your Linux Terminal",
        "   without needing to enter the same \"complex\" commands over and over",
        "   (unless if your are lazy like me).",
        "\n   Cmdcreate will usually get smaller, more frequent updates than larger,",
        "   less frequent updates. You should update Cmdcreate often to stay up to",
        "   date.",
    ];

    for line in lines {
        println!("{line}");
    }
}

fn main() {
    let args = utils::return_args();

    if args.is_empty() {
        display_usage();
        return;
    }

    // For debugging
    if args_contains("--arg_count") {
        println!("DEBUG: ARG COUNT: {}\n", args.len() - 1); // Subtracts 1 to exclude debug flag
    }

    match args[0].as_str() {
        // Normal commands
        "create" => create::create(),
        "remove" => remove::remove(),
        "edit" => edit::edit(),
        "list" => list::list(),
        "search" => search::search(),
        "reset" => reset::reset(),

        // Arguments
        "--version" => println!("{PROJ_VER}"),
        "--credits" => println!("cmdcreate: Credits:\n\nCreator/Maintainer:   Meme Supplier"),
        "--supported_editors" => {
            println!("\nCurrent supported editors:\n");
            for option in edit::SUPPORTED_EDITORS {
                println!("{option}")
            }
        }
        "--get_offline_files" => {
            println!("Downloading offline files...");

            run_shell_command("mkdir -p ~/.local/share/cmdcreate/", || {
                println!("Error: Failed to create directory: \"~/.local/share/cmdcreate/\".");
                return;
            });
            // License
            run_shell_command("curl -o ~/.local/share/cmdcreate/LICENSE https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/LICENSE", || {
                println!("Error: Unable to retrieve license.");
                return;
            });
            // Changelog
            run_shell_command("curl -o ~/.local/share/cmdcreate/changes.md https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/changes.md", || {
                println!("Error: Unable to retrieve changelog.");
                return;
            });

            println!("Files downloaded successfully.");
        }

        "--remove_offline_files" => run_shell_command(
            "rm -f ~/.local/share/cmdcreate/changes.md ~/.local/share/cmdcreate/LICENSE",
            || {
                println!("Error: Unable to remove files.")
            },
        ),

        "--license" => run_shell_command(
            "curl -s https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/LICENSE",
            || {
                run_shell_command("cat ~/.local/share/cmdcreate/LICENSE", || {
                    println!("Error: Unable to display license.");
                });
            },
        ),

        "--changelog" => run_shell_command(
            "curl -s https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/changes.md",
            || {
                run_shell_command("cat ~/.local/share/cmdcreate/changes.md", || {
                    println!("Error: Unable to display changelog.");
                });
            },
        ),

        "--debugging" => {
            let lines: [&str; 3] = [
                "cmdcreate: Methods for debugging:",
                "  --arg_count             Displays number of arguments supplied",
                "  --force_system_shell    Forces system shell to be used when running commands",
            ];

            for line in lines {
                println!("{line}")
            }
        }

        _ => println!("cmdcreate: Invalid command entered."),
    }
}
