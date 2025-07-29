use std::env;

mod tools;
use tools::{create, edit, list, remove};

static PROJ_VER: &str = "v0.4.2";

fn return_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn display_usage() {
    let lines: [&str; 20] = [
        &format!("cmdcreate {PROJ_VER}",),
        "",
        "Commands:",
        "  create <command> <contents>    Create a custom command",
        "  remove <command>               Remove a custom command",
        "  edit   <command>               Modify a custom command",
        "  list                           Display installed commands",
        "",
        "Flags:",
        "  --version                      Displays cmdcreate's version",
        "  --supported_editors            Displays supported text editors",
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
        "--version" if args.len() == 1 => println!("{PROJ_VER}"),
        "--supported_editors" if args.len() == 1 => {
            println!("\nCurrent supported editors:\n");
            for option in edit::SUPPORTED_EDITORS {
                println!("{option}")
            }
        }

        _ => display_usage()
    }
}
