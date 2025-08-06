mod tools;
use tools::*;

use crate::tools::utils::run_shell_command;

static PROJ_VER: &str = "v0.4.6";

fn display_usage() {
    let lines: [&str; 24] = [
        &format!("cmdcreate {PROJ_VER}",),
        "",
        "Commands:",
        "  create <command> <contents>    Create a custom command",
        "  remove <command>               Remove a custom command",
        "  edit   <command>               Modify a custom command",
        "  list                           Display installed commands",
        "  search <command>               Searches for matched command",
        "  reset                          Removes all installed commands",
        "",
        "Flags:",
        "  --version                      Displays version",
        "  --supported_editors            Displays supported text editors",
        "  --changelog                    Displays changelog",
        "  --license                      Displays license",
        "",
        "About:",
        "   Cmdcreate allows you to create custom commands for your Linux Terminal",
        "   without needing to enter the same \"complex\" commands over and over",
        "   (unless if your are lazy like me).",
        "",
        "   Cmdcreate will usually get smaller, more frequent updates than larger,",
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

    match args[0].as_str() {
        // Normal commands
        "create" if args.len() >= 2 => create::create(),
        "remove" if args.len() >= 1 => remove::remove(),
        "edit" if args.len() >= 1 => edit::edit(),
        "list" if args.len() == 1 => list::list(),
        "search" if args.len() >= 1 => search::search(),
        "reset" if args.len() == 1 => reset::reset(),

        // Flags
        "--version" => println!("{PROJ_VER}"),
        "--changelog" => run_shell_command(
            "curl -s https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/changes.md || echo -e \"\nError: Unable to retrieve changelog.\"",
        ),
        "--license" => run_shell_command(
            "curl -s https://raw.githubusercontent.com/Meme-Supplier/cmdcreate/master/LICENSE || echo -e \"\nError: Unable to retrieve license.\"",
        ),
        "--supported_editors" => {
            println!("\nCurrent supported editors:\n");
            for option in edit::SUPPORTED_EDITORS {
                println!("{option}")
            }
        }

        _ => display_usage(),
    }
}
