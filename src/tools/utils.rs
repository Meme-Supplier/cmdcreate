use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::{env, fs};

use crate::tools::upgrader::upgrade;
use crate::PROJ_VER;

pub fn retrieve_commands(val: &str) -> Vec<PathBuf> {
    let install_dir = dirs::home_dir()
        .expect("Home dir not found")
        .join(".local/share/cmdcreate/files");

    if !install_dir.exists() {
        println!("Error: No installed commands found.");
        return Vec::new();
    }

    if val == "dir" {
        return vec![install_dir];
    }

    let installed_scripts: Vec<PathBuf> = fs::read_dir(&install_dir)
        .expect("Error: Failed to read install directory")
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
        println!("Error: No installed commands found.");
        return Vec::new();
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

pub fn args_contains(s: &str) -> bool {
    return_args().contains(&s.to_string())
}

pub fn get_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| "unknown".to_string())
}

pub fn run_shell_command<F>(cmd: &str, fallback: F)
where
    F: FnOnce(),
{
    let shell: String;
    if args_contains("--force_system_shell") {
        shell = get_shell();
    } else {
        shell = "bash".to_string();
    }

    if cmd.trim().is_empty() {
        return;
    }

    match Command::new(shell)
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
        .expect("Error: Home dir not found")
        .join(".local/share/cmdcreate/files");

    if !input_path.starts_with(&allowed_root) {
        let filename = input_path.file_name().unwrap_or_default();
        allowed_root.join(filename)
    } else {
        input_path.to_path_buf()
    }
}

pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-client") // GitHub requires a User-Agent
        .send()?;

    let json: Value = response.json()?;
    let tag_name = json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name")?
        .to_string();

    Ok(tag_name)
}

pub fn check_for_updates() {
    println!("\nChecking for updates...");
    let current_ver = PROJ_VER;
    match get_latest_tag("Meme-Supplier", "cmdcreate") {
        Ok(latest_release) => {
            if current_ver != latest_release {
                println!("\x1b[32mUpdate available: {current_ver} -> {latest_release}\x1b[0m");
                upgrade();
            } else {
                println!("cmdcreate is already up to date: {current_ver}")
            }
        }
        Err(e) => eprintln!("Failed to check latest release: {e}\nTry making sure you're connected to the internet."),
    }
}

pub fn ask_for_confirmation(q: &str) {
    println!("{q}\n(Y or N)");
    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm).unwrap();
    if confirm.trim().to_lowercase() != "y" {
        println!("\nAborted.");
        exit(0);
    }
}
