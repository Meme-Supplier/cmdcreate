use std::process::exit;

use crate::utils::{
    colors::COLORS,
    fs::read_file_to_string,
    sys::{return_args, VARS},
};

pub fn display() {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}");
        exit(0);
    }
    let cmd = args.get(1).unwrap();

    println!(
        "Contents of command: {blue}\"{cmd}\"{reset}\n--------\n{}",
        read_file_to_string(&format!("{}/.local/share/cmdcreate/files/{cmd}", VARS.home)).trim()
    );
}
