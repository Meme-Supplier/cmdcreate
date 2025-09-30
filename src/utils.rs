use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::{env, fs};

use crate::cmds::upgrader::upgrade;

pub static PROJ_VER: &str = "v0.6.4";

pub const SUPPORTED_EDITORS: [&str; 13] = [
    "nvim",
    "vi",
    "vim",
    "nano",
    "micro",
    "code",
    "code-insiders",
    "gedit",
    "kate",
    "kwrite",
    "emacs",
    "vscodium",
    "vscodium-insiders",
];

pub fn retrieve_commands(val: &str) -> Vec<PathBuf> {
    let install_dir = dirs::home_dir()
        .unwrap_or_else(|| panic!("{}Error: Home dir not found{}", COLORS.red, COLORS.reset))
        .join(".local/share/cmdcreate/files");

    if !install_dir.exists() {
        error("No commands are installed.", "");
        return Vec::new();
    }

    if val == "dir" {
        return vec![install_dir];
    }

    let installed_scripts: Vec<PathBuf> = fs::read_dir(&install_dir)
        .unwrap_or_else(|_| {
            panic!(
                "{}Error: Failed to read install directory{}",
                COLORS.red, COLORS.reset
            )
        })
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
        error("No commands are installed.", "");
        return Vec::new();
    }

    if val == "installed" {
        installed_scripts
    } else {
        Vec::new()
    }
}

pub fn is_command_installed(cmd: &str) {
    let mut count: i32 = 0;
    for script in retrieve_commands("installed") {
        if script.file_stem().unwrap_or_default().to_string_lossy() == *cmd {
            count += 1
        }
    }

    if count == 0 {
        error("Command not installed: ", cmd);
        exit(0)
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

pub fn get_home() -> String {
    env::var("HOME").unwrap_or_else(|_| "unknown".to_string())
}

pub fn run_shell_command<F>(cmd: &str, fallback: F)
where
    F: FnOnce(),
{
    let shell: String = if args_contains("--force_system_shell") {
        get_shell()
    } else {
        "bash".to_string()
    };

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
                fallback();
            }
        }
        Err(e) => {
            error("", &e.to_string());
            fallback();
        }
    }
}

pub fn force_local_path<P: AsRef<Path>>(input: P) -> PathBuf {
    let input_path = shellexpand::tilde(input.as_ref().to_str().unwrap()).to_string();
    let input_path = Path::new(&input_path);

    let allowed_root = dirs::home_dir()
        .unwrap_or_else(|| panic!("{}Error: Home dir not found{}", COLORS.red, COLORS.reset))
        .join(".local/share/cmdcreate/files");

    if !input_path.starts_with(&allowed_root) {
        allowed_root.join(input_path.file_name().unwrap_or_default())
    } else {
        input_path.to_path_buf()
    }
}

pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json: Value = reqwest::blocking::Client::new()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .header("User-Agent", "rust-client")
        .send()?
        .json()?;

    Ok(json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name")?
        .to_string())
}

pub fn check_for_updates() {
    println!("\nChecking for updates...");
    match get_latest_tag("Meme-Supplier", "cmdcreate") {
        Ok(latest_release) => {
            if PROJ_VER != latest_release {
                println!(
                    "{}Update available: {PROJ_VER} -> {latest_release}{}",
                    COLORS.green, COLORS.reset
                );
                upgrade();
                return;
            }

            println!("cmdcreate is already up to date: {PROJ_VER}")
        }
        Err(e) => error("Failed to check latest release:", &e.to_string()),
    }
}

pub fn ask_for_confirmation(q: &str) {
    println!("{q}\n(Y or N)");
    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm).unwrap();
    if confirm.trim().to_lowercase() != "y" {
        println!("{}\nAborted.{}", COLORS.red, COLORS.reset);
        exit(0);
    }
}

pub fn error(msg: &str, err: &str) {
    eprintln!("{}Error: {msg} {err}{}", COLORS.red, COLORS.reset)
}

pub fn is_editor_installed(editor: &str) -> bool {
    Command::new("which")
        .arg(editor)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn read_file_to_string(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            error("Error reading file: ", &format!("\"{file_path}\": {e}"));
            String::new()
        }
    }
}

pub struct Colors {
    pub reset: &'static str,
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
}

pub const COLORS: Colors = Colors {
    reset: "\x1b[0m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    magenta: "\x1b[35m",
    cyan: "\x1b[36m",
};
