//! Favorite Command Management
//!
//! This module handles marking commands as “favorites” so users can
//! easily keep track of their most-used custom commands.
//!
//! # Features
//! - `favorite add <command>` — Add a command to the favorites list
//! - `favorite remove <command>` — Remove a command from the favorites list
//!
//! Favorites are stored in:
//! `~/.local/share/cmdcreate/favorites`

use crate::{
    cmds::tools::is_command_installed,
    utils::{
        colors::COLORS,
        fs::{create_file, remove_from_file, write_to_file},
        sys::{return_args, VARS},
    },
};

/// Entry point for the `favorite` command.
///
/// Parses arguments and routes to either [`add()`] or [`remove()`].
pub fn favorite() {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);
    let args = return_args();

    if args.len() < 2 || !["add", "remove"].contains(&args[1].as_str()) {
        println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}");
        return;
    }

    match args[1].as_str() {
        "add" => add(),
        "remove" => remove(),
        _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
    }
}

/// Adds a command to the favorites list.
///
/// # Behavior
/// - Validates command exists
/// - Ensures favorites file exists
/// - Appends command to favorites file
fn add() {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);
    let args = return_args();

    let command = &args[2];
    let path = format!("{}/.local/share/cmdcreate/favorites", VARS.home);

    is_command_installed(command);
    create_file(&path);
    write_to_file(&path, command);

    println!("{green}Command {blue}\"{command}\"{green} added to favorites.{reset}");
}

/// Removes a command from the favorites list.
///
/// # Behavior
/// - Validates command exists
/// - Removes command from favorites file if present
fn remove() {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);
    let args = return_args();

    let command = &args[2];
    let path = format!("{}/.local/share/cmdcreate/favorites", VARS.home);

    is_command_installed(command);
    remove_from_file(&path, command);

    println!("{green}Command {blue}\"{command}\"{green} removed from favorites.{reset}");
}
