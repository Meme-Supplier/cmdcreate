use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

pub fn retrieve_commands(val: &str) -> Vec<PathBuf> {
    let install_dir = dirs::home_dir()
        .expect("Home dir not found")
        .join(".local/share/cmdcreate/files");

    if !install_dir.exists() {
        println!("No installed commands found (directory doesn't exist)");
        return Vec::new();
    }

    if val == "dir" {
        return vec![install_dir];
    }

    let installed_scripts: Vec<PathBuf> = fs::read_dir(&install_dir)
        .expect("Failed to read install directory")
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if installed_scripts.is_empty() {
        println!("No installed commands found in: {}", install_dir.display());
    }

    if val == "installed" {
        installed_scripts
    } else {
        Vec::new()
    }
}

pub fn return_args() -> Vec<String> {
    env::args().skip(1).collect()
}

pub fn run_shell_command<F>(cmd: &str, fallback: F)
where
    F: FnOnce(),
{
    if cmd.trim().is_empty() {
        return;
    }

    match Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) => {
            if !status.success() {
                // Command ran but returned a non-zero exit code
                fallback();
            }
        }
        Err(e) => {
            eprintln!("{e}");
            fallback(); // Command failed to even run
        }
    }
}

pub fn force_local_path<P: AsRef<Path>>(input: P) -> PathBuf {
    let input_path = shellexpand::tilde(input.as_ref().to_str().unwrap()).to_string();
    let input_path = Path::new(&input_path);

    let allowed_root = dirs::home_dir()
        .expect("Home dir not found")
        .join(".local/share/cmdcreate/files");

    if !input_path.starts_with(&allowed_root) {
        let filename = input_path.file_name().unwrap_or_default();
        allowed_root.join(filename)
    } else {
        input_path.to_path_buf()
    }
}
