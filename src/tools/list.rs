use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::tools::utils::retrieve_commands;

pub fn list() {
    let installed_scripts = retrieve_commands("installed");

    let canonicalize_or = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let mut script_map = HashMap::new();

    for script_path in &installed_scripts {
        let abs_path = canonicalize_or(script_path);
        script_map.insert(
            abs_path,
            script_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        );
    }

    for script in installed_scripts {
        let command = script.file_stem().unwrap_or_default().to_string_lossy();
        println!("Installed commands:\n{command}")
    }
}
