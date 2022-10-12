use std::env::{join_paths, split_paths};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

/// A script has a command and a comment.
pub struct Script<'a> {
    /// The command of the script.
    command: &'a str,
    /// The comment for the script.
    comment: &'a str,
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
            comment: "This script has no comment.",
        }
    }

    /// Creates a new `Script`.
    /// # Usage
    /// ```rs
    /// let script_greet = Script::new("echo hey!", "Prints a greeting message.");
    /// ```
    pub fn new_with_comment(command: &'a str, comment: &'a str) -> Script<'a> {
        Script { command, comment }
    }

    /// Returns `self.command`.
    pub fn command(&self) -> &'a str {
        self.command
    }

    /// Returns `self.comment`.
    pub fn comment(&self) -> &'a str {
        self.comment
    }

    /// Executes `self.command`, then returns the exit code.
    pub fn execute(&self, extra_path: Option<&'static str>) -> i32 {
        // Get default shell program and c.
        let (shell, c) = get_shell_and_c();

        // Get $PATH variable with `extra_path`, if `extra_path` is not `None`.
        let path_var = extra_path.and_then(|extra_path| get_path_var_with(extra_path));

        // Create a `Command` for launching shell program.
        let mut command = Command::new(shell);

        // If `extra_path` is specified, set "PATH" environment variable to `path_var`.
        // Else, continue with the default environment variables.
        match path_var {
            Some(path_var) => command.arg(c).arg(&self.command).env("PATH", path_var),
            None => command.arg(c).arg(&self.command),
        }
        .status()
        .and_then(|status| Ok(status.code().unwrap_or(1)))
        .unwrap_or(1)
    }
}

/// Returns shell and c based on client OS.
fn get_shell_and_c() -> (&'static str, &'static str) {
    if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    }
}

/// Adds `extra_path` to `$PATH` environment variable, then returns it.
fn get_path_var_with(extra_path: &'static str) -> Option<OsString> {
    let mut paths: Vec<PathBuf> = split_paths(&std::env::var_os("PATH")?).collect();
    paths.push(PathBuf::from(extra_path));
    join_paths(paths).ok()
}
