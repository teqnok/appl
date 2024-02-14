#![allow(deprecated)]
use std::env;
use std::str::FromStr;
use toml_edit::Document;
pub fn get_appl_dir<T: ToString>(append: T) -> Option<String> {
    let mut appl_dir = String::new();
    #[cfg(unix)]
    {
        if let Some(mut dir) = env::home_dir() {
            dir.push(".appl/");
            appl_dir = dir.to_str()?.into();
        }
    }
    #[cfg(windows)]
    {
        if let Some(mut dir) = env::var_os("USERPROFILE").map(|p| PathBuf::from(p)) {
            dir.push(r"\AppData\Local\appl\");
            appl_dir = dir;
        }
    }
    appl_dir.push_str(append.to_string().as_str());
    return Some(appl_dir);
}
pub fn get_appl_config() -> Option<String> {
    let mut config_path = String::new();
    #[cfg(unix)]
    {
        if let Some(mut dir) = env::home_dir() {
            dir.push(".appl/config/appl.toml");
            config_path = dir.to_str()?.into();
        }
    }
    #[cfg(windows)]
    {
        if let Some(mut dir) = env::var_os("USERPROFILE").map(|p| PathBuf::from(p)) {
            dir.push(r"\AppData\Local\appl\config\appl.toml");
            config_path = dir;
        }
    }

    return Some(config_path);
}
/// Get a value from the local config file.
pub fn get_config_value<T: ToString>(table: T, value: T) -> Result<String, String> {
    let config_path = get_appl_config().unwrap_or("~/.appl/config/".into());
    let result: Result<String, String>;

    let toml_str = std::fs::read_to_string(config_path).expect("Unable to read config file");
    let doc = Document::from_str(&toml_str).expect("Unable to parse TOML!");
    if let Some(value) = doc[table.to_string().as_str()].get(value.to_string().as_str()) {
        result = Ok(value.to_string().trim().to_string().replace('"', ""));
    } else {
        result = Err("Called get_config_value on a non-existant value!".into());
    };

    return result;
}
pub fn get_toml_value<T: ToString>(file: T, table: T, value: T) -> Result<String, String> {
    let file = file.to_string();
    let result: Result<String, String>;
    let doc = Document::from_str(&file).expect("Unable to parse TOML");
    if let Some(value) = doc[table.to_string().as_str()].get(value.to_string().as_str()) {
        result = Ok(value.to_string());
    } else {
        result = Err("Called get_toml_value on a non-existant value!".into());
    }
    result
}
pub fn get_config_table<T: ToString>(table: T) -> Result<toml_edit::Item, String> {
    let config_path = get_appl_config().unwrap_or("~/.appl/config/".into());
    let toml_str = std::fs::read_to_string(config_path).expect("unable to read config file");
    let doc = Document::from_str(&toml_str).expect("unable to parse toml!");

    match doc.get(table.to_string().as_str()) {
        Some(item) => Ok(item.clone()),
        None => Err(format!(
            "Key '{}' does not exist in the configuration.",
            table.to_string()
        )),
    }
}
