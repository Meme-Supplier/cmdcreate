mod cmds;
mod utils;

use crate::{
    cmds::{edit::*, upgrader::*, *},
    utils::{colors::*, fs::*, msgs::*, sys::*},
};

pub static PROJ_VER: &str = "v0.6.9";

fn init_files() {
    run_shell_command(
        "
        curl -sSo ~/.local/share/cmdcreate/LICENSE https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/LICENSE; \
        curl -sSo ~/.local/share/cmdcreate/changes.md https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/changes.md
        "
    );
}

fn main() {
    let (magenta, green, reset) = (COLORS.magenta, COLORS.green, COLORS.reset);

    let args = return_args();
    if args.is_empty() {
        utils::usage::display_usage();
        return;
    }

    init_files();

    match args[0].as_str() {
        "create" => create::create(),
        "remove" => remove::remove(),
        "edit" => edit::edit(),
        "list" => list::list(),
        "search" => search::search(),
        "display" => display::display(),
        "rename" => rename::rename(),
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

            init_files();

            println!("{green}Files downloaded successfully.{reset}");
        }

        "--remove_offline_files" | "-r" => {
            delete_file(&format!("{}/.local/share/cmdcreate/changes.md", VARS.home));
            delete_file(&format!("{}/.local/share/cmdcreate/LICENSE", VARS.home));

            println!("Files removed successfully.");
        }

        "--license" | "-l" => {
            println!(
                "{}",
                read_file_to_string(&format!("{}/.local/share/cmdcreate/LICENSE", VARS.home))
            );
        }

        "--changelog" | "-c" => {
            println!(
                "{}",
                read_file_to_string(&format!("{}/.local/share/cmdcreate/changes.md", VARS.home))
                    .trim()
            );
        }

        "--debugging" | "-d" => {
            let lines: Vec<String> = vec![
                format!("Usage: cmdcreate {magenta}(flags){reset} [run] {magenta}(flags){reset}"),
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
