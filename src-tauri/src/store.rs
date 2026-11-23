//! Microsoft Store (MSIX) installs. The Store signs and updates the package
//! itself, and its policy (10.2.2) forbids downloading code that changes what
//! the app does, so a Store install never downloads cores (they ship in the
//! package with wintun and geodata) and never looks for app updates on GitHub.
//! The same build also runs as the installer and portable editions, so this is
//! decided at run time: Store packages are installed under `WindowsApps`.

use std::sync::OnceLock;

/// Message shown when a Store install is asked to download a core.
pub const STORE_NO_DOWNLOAD: &str =
    "Microsoft Store edition: cores ship with the app and are updated through the Store / 应用商店版：内核随应用提供，通过 Microsoft Store 更新";

/// True when running from an MSIX package installed by the Microsoft Store.
/// `DRVPN_STORE_BUILD=1` forces it (for testing the Store behavior elsewhere).
pub fn is_store_package() -> bool {
    static STORE: OnceLock<bool> = OnceLock::new();
    *STORE.get_or_init(|| {
        if std::env::var("DRVPN_STORE_BUILD").is_ok_and(|v| v == "1") {
            return true;
        }
        std::env::current_exe()
            .map(|exe| path_is_store_package(&exe.to_string_lossy()))
            .unwrap_or(false)
    })
}

fn path_is_store_package(exe: &str) -> bool {
    cfg!(windows) && exe.to_ascii_lowercase().replace('/', "\\").contains("\\windowsapps\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_package_paths() {
        let store = r"C:\Program Files\WindowsApps\DrVPN.DrVPN_1.0.48.0_x64__abc\drvpn-desktop.exe";
        let installer = r"C:\Program Files\DrVPN\drvpn-desktop.exe";
        assert_eq!(path_is_store_package(store), cfg!(windows));
        assert!(!path_is_store_package(installer));
        assert!(!path_is_store_package("/usr/bin/drvpn-desktop"));
    }
}
