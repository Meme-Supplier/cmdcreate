use std::process::exit;

use crate::tools::utils::*;

pub fn display() {
    let args = return_args();

    if args.len() < 2 {
        println!("Usage:\ncmdcreate display <command>");
        exit(0);
    }

    let cmd = args.get(1).unwrap();

    println!(
        "Contents of command: \"{cmd}\"\n--------\n{}",
        read_file_to_string(&format!(
            "{}/.local/share/cmdcreate/files/{cmd}",
            get_home()
        ))
        .trim()
    );
}
