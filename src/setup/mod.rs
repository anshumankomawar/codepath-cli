mod linux;
mod mac;
mod utils;
mod windows;

use linux::LinuxInstaller;
use log::info;
use mac::MacOSInstaller;
use windows::WindowsInstaller;

trait PlatformInstaller {
    fn install_python(&self) -> Result<(), String>;
    fn install_editor(&self, editor: &str) -> Result<(), String>;
    fn configure_paths(&self) -> Result<(), String>;
}

struct Installer {
    platform: Box<dyn PlatformInstaller>,
}

impl Installer {
    fn new() -> Self {
        let platform = std::env::consts::OS;
        info!("Detected platform: {}", platform);
        let platform_installer: Box<dyn PlatformInstaller> = match platform {
            "windows" => Box::new(WindowsInstaller),
            "linux" => Box::new(LinuxInstaller),
            "macos" => Box::new(MacOSInstaller),
            _ => panic!("Unsupported platform"),
        };
        Installer {
            platform: platform_installer,
        }
    }

    fn setup_environment(&self) -> Result<(), String> {
        self.platform.install_python()?;
        self.platform.install_editor("sublime")?;
        self.platform.configure_paths()?;
        Ok(())
    }
}

pub fn setup() {
    let installer = Installer::new();
    info!("Setting up your dev environment...");
    installer.setup_environment().unwrap();
}
