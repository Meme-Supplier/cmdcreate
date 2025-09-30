use crate::utils::*;

pub fn search() {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}search {yellow}<command>{reset}");
        return;
    }

    if let Some(name) = args.get(1) {
        let installed_scripts = retrieve_commands("installed");

        if installed_scripts.is_empty() {
            return;
        }

        let mut count: i32 = 0;
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
