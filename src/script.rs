use std::process::Command;

/// A script has a command and an info.
pub struct Script {
    /// The command of the script.
    command: String,
    /// The info about the script.
    info: String,
}

impl Script {
    /// Creates a new `Script`.
    /// # Usage
    /// ```rs
    /// let script = Script::new("echo hello");
    /// ```
    pub fn new(command: String) -> Script {
        Script {
            command,
            info: "this script has no info".to_string(),
        }
    }

    /// Creates a new `Script`.
    /// # Usage
    /// ```rs
    /// let script_greet = Script::new("echo hello", "Prints a greeting message.");
    /// ```
    pub fn new_with_info(command: String, info: String) -> Script {
        Script { command, info }
    }

    /// Executes `Script`. Returns its exit code.
    pub fn execute(&self) -> i32 {
        let (shell, c) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };
        Command::new(shell)
            .arg(c)
            .arg(&self.command)
            .status()
            .and_then(|status| Ok(status.code().unwrap_or(1)))
            .unwrap_or(1)
    }
}
