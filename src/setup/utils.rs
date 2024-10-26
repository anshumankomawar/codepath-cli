use std::process::{Command, Output};

pub fn run_command(cmd: &str, args: &[&str]) -> Result<Output, String> {
    Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))
}
