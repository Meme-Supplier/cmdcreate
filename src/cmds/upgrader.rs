use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::process::Command;

use crate::utils::colors::COLORS;
use crate::utils::fs::delete_folder;
use crate::utils::msgs::{ask_for_confirmation, error};
use crate::utils::sys::{run_shell_command, VARS};
use crate::PROJ_VER;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn upgrade() {
    let (blue, red, yellow, reset) = (COLORS.blue, COLORS.red, COLORS.yellow, COLORS.reset);

    let latest_release = get_latest_release();

    ask_for_confirmation("Do you want to upgrade cmdcreate?");

    println!(
        "\nSelect an upgrade method:\n\n{blue}1]{reset} Upgrade through AUR\n{blue}2]{reset} Install via .deb file\n{blue}3]{reset} Manually install raw binary"
    );

    let mut method = String::new();
    std::io::stdin().read_line(&mut method).unwrap();

    match method.trim() {
        "1" => {
            println!(
                "\nSelect a download method:\n\n{blue}1]{reset} AUR Manager {yellow}(yay {red}and {yellow}paru {red}ONLY)\n{blue}2]{reset} Clone repository {blue}(Recommended for when the AUR is down){reset}"
            );

            let mut method = String::new();
            std::io::stdin().read_line(&mut method).unwrap();

            match method.trim() {
                "1" => {
                    println!(
                        "\nSelect an AUR Manager:\n\n{blue}1]{reset} yay\n{blue}2]{reset} paru\n{blue}"
                    );

                    let mut method = String::new();
                    std::io::stdin().read_line(&mut method).unwrap();

                    match method.trim() {
                        "1" | "2" => run_shell_command(&format!("{method} -Syyu"), || {}),
                        _ => error("Invalid selection.", ""),
                    }
                }
                "2" => {
                    run_shell_command(
                            "
                            sudo rm /usr/bin/cmdcreate; \
                            git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git cmdcreate; \
                            cd ~/cmdcreate; \
                            makepkg -si", || error("Failed installing package.",""));
                    delete_folder(&format!("{}/cmdcreate", VARS.home));
                }

                _ => error("Invalid selection.", ""),
            }
        }
        "2" => {
            println!("Downloading latest Debian package...");

            run_shell_command(
                &format!(
                    "
                    curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.deb https://github.com/Meme-Supplier/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.deb; \
                    sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux-x86_64.deb;
                    "
                ), || error("Failed to install latest Debian package.", ""));
        }
        "3" => {
            println!("Downloading latest binary...");

            let (owner, repo) = ("Meme-Supplier", "cmdcreate");
            let latest_release = get_latest_release();
            let file_to_download = format!("cmdcreate-{latest_release}-linux-bin");
            let client = Client::new();

            let release: Release = client
                .get(format!(
                    "https://api.github.com/repos/{owner}/{repo}/releases/latest"
                ))
                .header("User-Agent", "reqwest")
                .send()
                .unwrap()
                .json()
                .unwrap();

            if let Some(asset) = release
                .assets
                .into_iter()
                .find(|a| a.name == file_to_download)
            {
                let tmp_path = format!("/tmp/{}", asset.name);
                let mut tmp_file = File::create(&tmp_path).unwrap();
                std::io::copy(
                    &mut client.get(&asset.browser_download_url).send().unwrap(),
                    &mut tmp_file,
                )
                .unwrap();

                let out_path = format!("/usr/bin/{}", asset.name);
                Command::new("sudo")
                    .args(["mv", &tmp_path, &out_path])
                    .status()
                    .unwrap();

                Command::new("sudo")
                    .args(["chmod", "+x", &out_path])
                    .status()
                    .unwrap();

                run_shell_command(&format!("sudo rm /usr/bin/cmdcreate; sudo mv /usr/bin/{file_to_download} /usr/bin/cmdcreate"), || {});

                println!(
                    "Downloaded {} from release {}",
                    asset.name, release.tag_name
                );
            } else {
                error("Binary not found in latest release.", "");
            }
        }
        _ => error("Invalid selection.", ""),
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

pub fn get_latest_release() -> String {
    get_latest_tag("Meme-Supplier", "cmdcreate")
        .unwrap()
        .to_string()
}

pub fn check_for_updates() {
    println!("\nChecking for updates...");
    let latest_release = get_latest_release();
    if PROJ_VER != latest_release {
        println!(
            "{}Update available: {PROJ_VER} -> {latest_release}{}",
            COLORS.green, COLORS.reset
        );
        upgrade();
    }
}
