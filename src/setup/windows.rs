use super::{utils::run_command, PlatformInstaller};

/// Check if Chocolatey is installed, and install if it isn't
fn ensure_chocolatey_installed() -> Result<(), String> {
    let output = run_command("choco", &["-v"]);
    if output.is_err() {
        println!("Chocolatey not found. Installing Chocolatey...");
        let install_cmd = "powershell";
        let install_args = &[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "Set-ExecutionPolicy Bypass -Scope Process; \
             [System.Net.ServicePointManager]::SecurityProtocol = \
             [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; \
             iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))",
        ];
        run_command(install_cmd, install_args)?;
    }
    Ok(())
}

// Windows-specific installer struct
pub struct WindowsInstaller;
impl PlatformInstaller for WindowsInstaller {
    fn install_python(&self) -> Result<(), String> {
        ensure_chocolatey_installed()?;
        println!("Installing Python...");
        run_command("choco", &["install", "python", "--version=3.9", "-y"])?;
        println!("Python installed successfully.");
        Ok(())
    }

    fn install_editor(&self, editor: &str) -> Result<(), String> {
        ensure_chocolatey_installed()?;
        let editor_package = match editor {
            "sublime" => "sublimetext",
            _ => return Err(format!("Unsupported editor: {}", editor)),
        };
        println!("Installing {}...", editor);
        run_command("choco", &["install", editor_package, "-y"])?;
        println!("{} installed successfully.", editor);
        Ok(())
    }

    fn configure_paths(&self) -> Result<(), String> {
        Ok(())
    }
}
