use crate::utils::*;

pub fn list() {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    let installed_scripts = retrieve_commands("installed");
    if installed_scripts.is_empty() {
        return;
    }

    println!(
        "Installed commands: ({blue}{}{reset})\n--------",
        installed_scripts.len()
    );

    for script in installed_scripts {
        println!(
            "{}",
            script.file_stem().unwrap_or_default().to_string_lossy()
        )
    }
}
