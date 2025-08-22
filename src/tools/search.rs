use crate::tools::utils::*;

pub fn search() {
    let args = return_args();

    if args.len() < 2 {
        println!("Usage:\ncmdcreate search <command>");
        return
    }

    if let Some(name) = args.get(1) {
        let installed_scripts = retrieve_commands("installed");

        if installed_scripts.is_empty() {
            return
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
            println!("No installed commands contain: {name}");
            return
        }

        println!("--------\nFound {count} commands that contain \"{name}\"")
    } else {
        crate::display_usage();
    }
}
