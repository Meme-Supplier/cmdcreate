use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};

pub fn run_shell_command(cmd: &str) {
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
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            exit(0)
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
