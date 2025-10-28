use crate::{utils::colors::COLORS, *};

pub fn display_usage() {
    let (blue, cyan, yellow, magenta, reset) = (
        COLORS.blue,
        COLORS.cyan,
        COLORS.yellow,
        COLORS.magenta,
        COLORS.reset,
    );

    let lines: Vec<String> = vec![
        format!("cmdcreate {PROJ_VER}"),
        format!("Usage: cmdcreate {magenta}(flags){reset} [{blue}command{reset}, {cyan}argument{reset}] {yellow}<args> {magenta}(flags){reset}"),
        "\nCommands:".into(),
        format!("  {blue}create{yellow}  <command> <contents>{reset}   Create a command"),
        format!("  {blue}remove {yellow} <command>{reset}              Remove a command"),
        format!("  {blue}edit   {yellow} <command>{reset}              Modify a command"),
        format!("  {blue}list{reset}                           Display installed commands"),
        format!("  {blue}search {yellow} <command>{reset}              Searches for matched command"),
        format!("  {blue}display {yellow}<command>{reset}              Display contents of a command"),
        format!("  {blue}rename {yellow} <command> <new name>{reset}   Renames a command"),
        "\n  Update:".into(),
        format!("    {blue}check{reset}                        Checks for updates"),
        format!("    {blue}update{reset}                       Updates your system"),
        "\nArguments:".into(),
        format!("  {cyan}-v{reset},{cyan} --version {reset}                 Displays version"),
        format!("  {cyan}-s{reset},{cyan} --supported_editors {reset}       Displays supported text editors"),
        format!("  {cyan}-c{reset},{cyan} --changelog {reset}               Displays changelog"),
        format!("  {cyan}-l{reset},{cyan} --license {reset}                 Displays license"),
        format!("  {cyan}-d{reset},{cyan} --debugging {reset}               Displays flags used for debugging"),
        "\n  Offline:".into(),
        format!("    {cyan}-g{reset},{cyan} --get_offline_files{reset}      Downloads files for offline use"),
        format!("    {cyan}-r{reset},{cyan} --remove_offline_files{reset}   Removes files for offline use"),
        "\nAbout:".into(),
        "   Cmdcreate allows you to create custom commands for your Linux Terminal".into(),
        "   without needing to enter the same \"complex\" commands over and over".into(),
        "   (unless if your are lazy like me).".into(),
        "\n   Cmdcreate will usually get smaller, more frequent updates than larger,".into(),
        "   less frequent updates. You should update Cmdcreate often to stay up to".into(),
        "   date.".into(),
    ];

    for line in lines {
        println!("{line}");
    }
}
