use std::{fs, process::exit};

use crate::tools::utils::*;

pub fn display() -> std::io::Result<()> {
    let args = return_args();

    if args.len() < 2 {
        println!("Usage:\ncmdcreate display <command>");
        exit(0);
    }

    let cmd = args.get(1).unwrap();

    println!(
        "Contents of command: \"{cmd}\"\n--------\n{}",
        fs::read_to_string(format!(
            "{}/.local/share/cmdcreate/files/{cmd}",
            get_home_dir().unwrap().to_string_lossy()
        ))?
        .trim()
    );
    Ok(())
}
