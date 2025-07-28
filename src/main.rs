use std::env;

mod tools;
use tools::{create, edit, list, remove};

static PROJ_VER: &str = "v0.4.0";

fn return_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn display_usage() {
    let lines: [&str; 13] = [
        &format!("cmdcreate {PROJ_VER}",),
        "",
        "Usage:",
        "",
        "create <name> <contents>    Create a custom command",
        "remove <name>               Remove a custom command",
        "edit <name>                 Modify a custom command",
        "list                        Display custom commands",
        "",
        "Flags:",
        "",
        "--version                   Displays cmdcreate's version",
        "--supported_editors         Displays supported text editors",
    ];

    for line in lines {
        println!("{line}");
    }
}

fn main() {
    let args = return_args();

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

        // Flags
        "--hepl" if args.len() == 1 => display_usage(),
        "--version" if args.len() == 1 => println!("{PROJ_VER}"),
        "--supported_editors" if args.len() == 1 => {
            println!("\nCurrent supported editors:\n");
            for option in edit::SUPPORTED_EDITORS {
                println!("{option}")
            }
        }

        _ => {
            eprintln!("Unknown command: '{}'", args[0]);
            display_usage();
        }
    }
}
