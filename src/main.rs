use std::env;

mod tools;
use tools::{create, list, remove};

fn return_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn display_usage() {
    let lines: [&str; 7] = [
        "cmdcreate v0.2",
        "",
        "Usage:",
        "",
        "create <name> <file> <contents>",
        "remove <name> <file>",
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
        "create" => create::create(),
        "remove" => remove::remove(),
        "list" => list::list(),
        _ => display_usage(),
    }
}
