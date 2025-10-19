use std::process::exit;

use crate::utils::{colors::COLORS, sys::args_contains};

pub fn ask_for_confirmation(q: &str) {
    if !args_contains("--force") && !args_contains("-f") {
        println!("{q}\n(Y or N)");
        let mut confirm = String::new();
        std::io::stdin().read_line(&mut confirm).unwrap();
        if confirm.trim().to_lowercase() != "y" {
            println!("{}\nAborted.{}", COLORS.red, COLORS.reset);
            exit(0)
        }
    }
}

pub fn error(msg: &str, err: &str) {
    eprintln!("{}Error: {} {err}{}", COLORS.red, msg.trim(), COLORS.reset);
    exit(1)
}
