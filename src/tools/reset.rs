use crate::tools::utils::*;

pub fn reset() {
    // Ask for confirmation
    println!(
        "Are you sure you want remove ALL installed custom commands?\nTHIS CAN NOT BE UNDONE (y/N)"
    );
    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm).unwrap();
    if confirm.trim().to_lowercase() != "y" {
        println!("Aborted.");
        return;
    }

    run_shell_command("rm -f ~/.local/share/cmdcreate/files/*", || {
        println!("Error: Unable to retrieve license.");
    });
}
