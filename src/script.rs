use std::process::Command;

/// A script has a command and an info.
pub struct Script<'a> {
    /// The command of the script.
    pub command: &'a str,
    /// The info about the script.
    pub info: &'a str,
}

impl<'a> Script<'a> {
    /// Creates a new `Script`.
    /// # Usage
    /// ```rs
    /// let script = Script::new("echo hello");
    /// ```
    pub fn new(command: &'a str) -> Script<'a> {
        Script {
            command,
            info: "this script has no info",
        }
    }

    /// Creates a new `Script`.
    /// # Usage
    /// ```rs
    /// let script_greet = Script::new("echo hello", "Prints a greeting message.");
    /// ```
    pub fn new_with_info(command: &'a str, info: &'a str) -> Script<'a> {
        Script { command, info }
    }

    /// Executes `Script`. Returns its exit code.
    pub fn execute(&self, path: &str) -> i32 {
        let (shell, c) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        Command::new(shell)
            .env("PATH", path)
            .arg(c)
            .arg(&self.command)
            .status()
            .and_then(|status| Ok(status.code().unwrap_or(1)))
            .unwrap_or(1)
    }
}
