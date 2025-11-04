/// Module for terminal color formatting in cmdcreate
///
/// This module provides ANSI escape codes for colored output in terminal
/// environments. It includes a set of commonly used colors and a reset code
/// to return to default terminal colors.
/// Structure containing ANSI escape codes for terminal colors
///
/// Each field represents a different color or formatting option:
/// - `reset`: Returns terminal color to default
/// - `red`: Used for errors and warnings
/// - `green`: Used for success messages
/// - `yellow`: Used for important values and highlights
/// - `blue`: Used for commands and actions
/// - `magenta`: Used for flags and options
/// - `cyan`: Used for auxiliary information
pub struct Colors {
    /// Reset terminal color to default
    pub reset: &'static str,
    /// Red color - for errors and warnings
    pub red: &'static str,
    /// Green color - for success messages
    pub green: &'static str,
    /// Yellow color - for important values
    pub yellow: &'static str,
    /// Blue color - for commands and actions
    pub blue: &'static str,
    /// Magenta color - for flags and options
    pub magenta: &'static str,
    /// Cyan color - for auxiliary information
    pub cyan: &'static str,
}

/// Global constant containing all available terminal colors
///
/// Usage example:
/// ```rust
/// println!("{}Success!{}", COLORS.green, COLORS.reset);
/// println!("{}Error:{} {}", COLORS.red, COLORS.reset, "Something went wrong");
/// ```
pub const COLORS: Colors = Colors {
    // ANSI escape code for resetting colors
    reset: "\x1b[0m",
    // ANSI escape code for red text
    red: "\x1b[31m",
    // ANSI escape code for green text
    green: "\x1b[32m",
    // ANSI escape code for yellow text
    yellow: "\x1b[33m",
    // ANSI escape code for blue text
    blue: "\x1b[34m",
    // ANSI escape code for magenta text
    magenta: "\x1b[35m",
    // ANSI escape code for cyan text
    cyan: "\x1b[36m",
};
