/************************************************
* File: config.rs
* Author: Michal Švrček
*
* DevDock session configuration and storage
*
* ver. 0.2.0
*************************************************/

use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub folder: PathBuf,

    #[serde(default)]
    pub vscode: Vec<PathBuf>,

    #[serde(default)]
    pub folders: Vec<PathBuf>,

    #[serde(default)]
    pub terminals: Vec<Terminal>,

    #[serde(default)]
    pub websites: Vec<String>,

    #[serde(default)]
    pub commands: Vec<CustomCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Terminal {
    pub path: PathBuf,

    #[serde(default)]
    pub shell: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomCommand {
    pub program: String,

    #[serde(default)]
    pub args: Vec<String>,

    #[serde(default)]
    pub cwd: Option<PathBuf>,
}

pub fn config_dir() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        let appdata = env::var_os("APPDATA").ok_or("APPDATA is not set")?;

        Ok(PathBuf::from(appdata).join("DevDock").join("sessions"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        let base = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .ok_or("Cannot determine configuration directory")?;

        Ok(base.join("devdock").join("sessions"))
    }
}

fn session_path(name: &str) -> Result<PathBuf, String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Invalid session name".into());
    }

    Ok(config_dir()?.join(format!("{name}.toml")))
}

pub fn add(name: &str, folder: &Path) -> Result<(), String> {
    let folder = folder
        .canonicalize()
        .map_err(|e| format!("Invalid folder: {e}"))?;

    if !folder.is_dir() {
        return Err("Path must be a directory".into());
    }

    let path = session_path(name)?;

    fs::create_dir_all(config_dir()?).map_err(|e| e.to_string())?;

    let session = Session {
        name: name.to_string(),
        folder,
        vscode: Vec::new(),
        folders: Vec::new(),
        terminals: Vec::new(),
        websites: Vec::new(),
        commands: Vec::new(),
    };

    let content = toml::to_string_pretty(&session).map_err(|e| e.to_string())?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| format!("Cannot create session '{name}': {e}"))?;

    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    println!("Session '{name}' created.");
    println!("Configuration: {}", path.display());

    Ok(())
}

pub fn load(name: &str) -> Result<Session, String> {
    let path = session_path(name)?;

    let content =
        fs::read_to_string(path).map_err(|e| format!("Cannot load session '{name}': {e}"))?;

    let session: Session =
        toml::from_str(&content).map_err(|e| format!("Invalid session configuration: {e}"))?;

    if session.name != name {
        return Err("Session name does not match filename".into());
    }

    Ok(session)
}

pub fn list() -> Result<(), String> {
    let dir = config_dir()?;

    if !dir.exists() {
        println!("No sessions configured yet.");
        return Ok(());
    }

    let mut names = Vec::new();

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();

        if path.extension().is_some_and(|ext| ext == "toml") {
            if let Some(name) = path.file_stem() {
                names.push(name.to_string_lossy().into_owned());
            }
        }
    }

    names.sort();

    if names.is_empty() {
        println!("No sessions configured yet.");
    } else {
        println!("Available sessions:");

        for name in names {
            println!("  {name}");
        }
    }

    Ok(())
}

pub fn edit(name: &str) -> Result<(), String> {
    let path = session_path(name)?;

    if !path.is_file() {
        return Err(format!("Session '{name}' does not exist"));
    }

    #[cfg(target_os = "windows")]
    let mut editor = {
        let executable = env::var_os("EDITOR").unwrap_or_else(|| "notepad.exe".into());

        Command::new(executable)
    };

    #[cfg(not(target_os = "windows"))]
    let mut editor = {
        let executable = env::var_os("EDITOR").unwrap_or_else(|| "vi".into());

        Command::new(executable)
    };

    let status = editor
        .arg(&path)
        .status()
        .map_err(|e| format!("Cannot open editor: {e}"))?;

    if !status.success() {
        return Err("Editor exited with an error".into());
    }

    Ok(())
}
