use std::env;

mod tools;
use tools::{create, list, remove};

fn return_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn display_usage() {
    let lines: [&str; 7] = [
        "cmdcreate v0.3",
        "",
        "Usage:",
        "",
        "create <name> <contents>",
        "remove <name>",
        "list",
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
        "create" if args.len() >= 2 => create::create(),
        "remove" if args.len() >= 1 => remove::remove(),
        "list" if args.len() == 1 => list::list(),
        _ => {
            eprintln!("Unknown command: '{}'", args[0]);
            display_usage();
        }
    }
}
