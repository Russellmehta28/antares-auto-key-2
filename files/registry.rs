//! Windows registry read helpers for Antares Auto Key 2.

pub const APP_KEY: &str = "Software\\Antares\\Antares Auto Key 2";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Antares Auto Key 2";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Antares Auto Key 2") },
        RegValue { name: "Version",     value: String::from("v3.6.0") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Antares Auto Key 2") },
        RegValue { name: "Publisher",       value: String::from("Antares") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Antares Auto Key 2\\uninstall.exe\"") },
    ]
}
