use crate::utils::*;

pub fn search() {
    let args = return_args();
    let mut count: i32 = 0;

    let blue = COLORS.blue;
    let yellow = COLORS.yellow;
    let reset = COLORS.reset;

    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}search {yellow}<command>{reset}");
        return;
    }

    if let Some(name) = args.get(1) {
        let installed_scripts = retrieve_commands("installed");

        if installed_scripts.is_empty() {
            return;
        }

        for script in installed_scripts {
            let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();

            if file_stem.contains(name) {
                println!("{file_stem}");
                count += 1;
            }
        }

        if count == 0 {
            error(
                "No installed commands contain",
                &format!("{yellow}\"{name}\"{reset}"),
            );
        }
    }
}
