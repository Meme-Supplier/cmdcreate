use crate::utils::*;

use std::process::exit;

pub fn display() {
    let args = return_args();

    let blue = COLORS.blue;
    let yellow = COLORS.yellow;
    let reset = COLORS.reset;

    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}");
        exit(0);
    }

    let cmd = args.get(1).unwrap();

    println!(
        "Contents of command: {blue}\"{cmd}\"{reset}\n--------\n{}",
        read_file_to_string(&format!(
            "{}/.local/share/cmdcreate/files/{cmd}",
            get_home()
        ))
        .trim()
    );
}
