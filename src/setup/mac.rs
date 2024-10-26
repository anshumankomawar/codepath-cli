use super::{utils::run_command, PlatformInstaller};

/// Check if Homebrew is installed, and install if it isn't
fn ensure_homebrew_installed() -> Result<(), String> {
    let output = run_command("brew", &["--version"]);
    if output.is_err() {
        println!("Homebrew not found. Installing Homebrew...");
        let install_cmd = "/bin/bash";
        let install_args = &[
            "-c",
            "\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"",
        ];
        run_command(install_cmd, install_args)?;
    }
    Ok(())
}

// MacOS-specific installer struct
pub struct MacOSInstaller;
impl PlatformInstaller for MacOSInstaller {
    fn install_python(&self) -> Result<(), String> {
        ensure_homebrew_installed()?;
        println!("Installing Python...");
        run_command("brew", &["install", "python@3.9"])?;
        println!("Python installed successfully.");
        Ok(())
    }
    fn install_editor(&self, editor: &str) -> Result<(), String> {
        ensure_homebrew_installed()?;
        let editor_package = match editor {
            "sublime" => "sublime-text",
            _ => return Err(format!("Unsupported editor: {}", editor)),
        };
        println!("Installing {}...", editor);
        run_command("brew", &["install", "--cask", editor_package])?;
        println!("{} installed successfully.", editor);
        Ok(())
    }
    fn configure_paths(&self) -> Result<(), String> {
        Ok(())
    }
}
