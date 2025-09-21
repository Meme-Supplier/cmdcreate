use crate::tools::utils::retrieve_commands;

pub fn list() {
    let installed_scripts = retrieve_commands("installed");

    if installed_scripts.is_empty() {
        return;
    }

    println!(
        "Installed commands: ({} installed)\n--------",
        installed_scripts.len()
    );

    for script in installed_scripts {
        println!(
            "{}",
            script.file_stem().unwrap_or_default().to_string_lossy()
        )
    }
}
