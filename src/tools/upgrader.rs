use std::fs::File;
use std::process::Command;

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{
    tools::utils::{ask_for_confirmation, get_latest_tag, run_shell_command},
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
    let current_ver = PROJ_VER;
    match get_latest_tag("Meme-Supplier", "cmdcreate") {
        Ok(latest_release) => {
            if current_ver == latest_release {
                println!("Already up to date: {current_ver}\nNo need to update.");
                return
            }
        }
        Err(e) => eprintln!("Failed to check latest release: {e}\nTry making sure you're connected to the internet."),
    }

    ask_for_confirmation("Do you want to upgrade cmdcreate?");

    println!(
        "\nSelect an upgrade method:\n\n1] Upgrade through AUR\n2] Install via .deb file\n3] Manually install raw binary"
    );

    let mut method = String::new();
    std::io::stdin().read_line(&mut method).unwrap();

    match method.trim() {
        "1" => run_shell_command("sudo rm /usr/bin/cmdcreate; git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git cmdcreate; cd ~/cmdcreate; makepkg -si; rm -rf ~/cmdcreate", || return),
        "2" => {
            println!("Downloading latest .deb and installing...");

            match get_latest_tag("Meme-Supplier", "cmdcreate") {
                Ok(latest_release) => {
                    run_shell_command(&format!("curl -L -o /tmp/cmdcreate-{latest_release}-linux.deb https://github.com/Meme-Supplier/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux.deb"), || return);
                    run_shell_command(&format!("sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux.deb"), || return);
                }
                Err(e) => eprintln!("Failed to check latest release: {e}\nTry making sure you're connected to the internet."),
            }
        }
        "3" => {
            println!("Downloading latest raw binary from GitHub...");

            let owner = "Meme-Supplier";
            let repo = "cmdcreate";

            // Get the latest release tag
            let latest_release = match get_latest_tag(owner, repo) {
                Ok(tag) => tag,
                Err(e) => {
                    eprintln!("Failed to check latest release: {e}");
                    return;
                }
            };

            let file_to_download = format!("cmdcreate-{latest_release}-linux-bin");

            let client = Client::new();
            let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
            let release: Release = client
                .get(&url)
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
                let mut resp = client.get(&asset.browser_download_url).send().unwrap();
                let tmp_path = format!("/tmp/{}", asset.name);
                let mut tmp_file = File::create(&tmp_path).unwrap();
                std::io::copy(&mut resp, &mut tmp_file).unwrap();

                let out_path = format!("/usr/bin/{}", asset.name);
                // Move to /usr/bin using sudo
                Command::new("sudo")
                    .args(["mv", &tmp_path, &out_path])
                    .status()
                    .unwrap();

                // Make it executable using sudo
                Command::new("sudo")
                    .args(["chmod", "+x", &out_path])
                    .status()
                    .unwrap();

                run_shell_command(&format!("sudo rm /usr/bin/cmdcreate; sudo mv /usr/bin/{file_to_download} /usr/bin/cmdcreate"), || return);

                println!(
                    "Downloaded {} from release {}",
                    asset.name, release.tag_name
                );
            } else {
                eprintln!("Error: Binary not found in latest release.");
            }
        }
        _ => eprintln!("Error: Invalid selection. Aborted."),
    }
}
