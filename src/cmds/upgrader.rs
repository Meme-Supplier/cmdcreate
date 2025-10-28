use std::{error::Error, fs::File};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    utils::{colors::*, fs::*, msgs::*, sys::*},
    PROJ_VER,
};

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

    let latest_release = get_latest_release().unwrap().to_string();

    ask_for_confirmation("Do you want to upgrade cmdcreate?");

    println!(
        "\nSelect an upgrade method:\n\n{blue}1]{reset} Upgrade through AUR\n{blue}2]{reset} Install via .deb file\n{blue}4]{reset} Manually install binary"
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
                        "1" | "2" => run_shell_command(&format!("{method} -Syyu")),
                        _ => error("Invalid selection.", ""),
                    }
                }
                "2" => {
                    run_shell_command(
                            "
                            sudo rm /usr/bin/cmdcreate; \
                            git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git cmdcreate; \
                            cd ~/cmdcreate; \
                            makepkg -si");
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
                ));
        }
        "3" => {
            println!("Downloading latest RPM package...");

            run_shell_command(
                &format!(
                    "
                    curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm https://github.com/Meme-Supplier/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.rpm; \
                    sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm;
                    "
                ));
        }
        "4" => {
            println!("Downloading latest binary...");

            let (owner, repo) = ("Meme-Supplier", "cmdcreate");
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
                let tmp_path = &format!("/tmp/{}", asset.name);
                let mut tmp_file = File::create(tmp_path).unwrap();
                let out_path = &format!("/usr/bin/{}", asset.name);

                std::io::copy(
                    &mut client.get(&asset.browser_download_url).send().unwrap(),
                    &mut tmp_file,
                )
                .unwrap();

                run_shell_command(&format!(
                    "
                        sudo mv +x {tmp_path} {out_path}; \
                        sudo chmod +x {out_path}; \
                        sudo rm /usr/bin/cmdcreate; \
                        sudo mv /usr/bin/{file_to_download} /usr/bin/cmdcreate
                        ",
                ));

                println!(
                    "Downloaded {} from release {}",
                    asset.name, release.tag_name
                )
            } else {
                error("Binary not found in latest release.", "");
            }
        }
        _ => error("Invalid selection.", ""),
    }
}

pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    let res = reqwest::blocking::Client::new()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .header("User-Agent", "rust-client")
        .send()?;

    let json: Value = res.json()?;
    let tag = json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name in response")?
        .to_string();

    Ok(tag)
}

pub fn get_latest_release() -> Option<String> {
    get_latest_tag("Meme-Supplier", "cmdcreate").ok()
}

pub fn check_for_updates() {
    println!("\nChecking for updates...");
    match get_latest_release() {
        Some(latest) if latest != PROJ_VER => {
            println!(
                "{}Update available: {PROJ_VER} -> {latest}{}",
                COLORS.green, COLORS.reset
            );
            upgrade();
        }
        Some(_) => println!("Already up to date."),
        None => println!(
            "{}Failed to fetch the latest release.{}",
            COLORS.red, COLORS.reset
        ),
    }
}
