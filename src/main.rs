mod cmds;
mod utils;

use crate::{
    cmds::{edit::SUPPORTED_EDITORS, upgrader::check_for_updates, *},
    utils::{
        fs::{delete_file, read_file_to_string},
        msgs::{ask_for_confirmation, error},
    },
};

use utils::{colors::COLORS, sys::*};

pub static PROJ_VER: &str = "v0.6.8";

fn main() {
    let (red, magenta, green, reset) = (COLORS.red, COLORS.magenta, COLORS.green, COLORS.reset);

    let args = return_args();
    if args.is_empty() {
        utils::usage::display_usage();
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
            delete_file(&format!("{}/.local/share/cmdcreate/changes.md", VARS.home));
            delete_file(&format!("{}/.local/share/cmdcreate/LICENSE", VARS.home));

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
