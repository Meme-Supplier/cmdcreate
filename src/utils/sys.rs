use std::{
    env,
    process::{Command, Stdio},
};

use once_cell::sync::Lazy;

use crate::utils::msgs::error;

pub struct Vars {
    pub shell: String,
    pub home: String,
}

pub static VARS: Lazy<Vars> = Lazy::new(|| Vars {
    shell: env::var("SHELL").unwrap_or_else(|_| "unknown".to_string()),
    home: env::var("HOME").unwrap_or_else(|_| "unknown".to_string()),
});

pub fn return_args() -> Vec<String> {
    env::args()
        .skip(1) // skip program name
        .collect()
}

pub fn args_contains(s: &str) -> bool {
    return_args().contains(&s.to_string())
}

pub fn run_shell_command(cmd: &str) {
    let shell: String = if args_contains("--force_system_shell") | args_contains("-F") {
        VARS.shell.clone()
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
        Ok(_) => {}
        Err(e) => {
            error("", &e.to_string());
        }
    }
}
