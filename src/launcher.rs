/************************************************
* File: launcher.rs
* Author: Michal Švrček
*
* DevDock workspace application launcher
*
* ver. 0.4.1
*************************************************/

use crate::config::{CustomCommand, Session, Terminal};
use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::output;

pub fn launch(session: &Session) -> Result<(), String> {
    output::banner();
    output::heading(&format!("Launching session: {}", session.name));
    output::step("Opening workspace applications");

    let mut errors = Vec::new();

    let projects: Vec<_> = if session.vscode.is_empty() {
        vec![&session.folder]
    } else {
        session.vscode.iter().collect()
    };

    for path in projects {
        if let Err(e) = open_vscode(path) {
            errors.push(e);
        }
    }

    for path in &session.folders {
        if let Err(e) = open_folder(path) {
            errors.push(e);
        }
    }

    for terminal in &session.terminals {
        if let Err(e) = open_terminal(terminal) {
            errors.push(e);
        }
    }

    for url in &session.websites {
        if let Err(e) = open_website(url) {
            errors.push(e);
        }
    }

    for command in &session.commands {
        if let Err(e) = run_custom_command(command) {
            errors.push(e);
        }
    }

    if errors.is_empty() {
        println!();
        output::success("Session launch completed");
        Ok(())
    } else {
        println!();

        for error in &errors {
            output::error(error);
        }

        Err(format!("{} session action(s) failed.", errors.len()))
    }
}

fn verify_directory(path: &Path) -> Result<(), String> {
    if !path.is_dir() {
        return Err(format!("Directory does not exist: {}", path.display()));
    }

    Ok(())
}

fn open_vscode(path: &Path) -> Result<(), String> {
    verify_directory(path)?;

    println!("  [VSCode] {}", path.display());

    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut command = Command::new("cmd");
        command.arg("/C").arg("code");
        command
    };

    #[cfg(not(target_os = "windows"))]
    let mut cmd = Command::new("code");

    cmd.arg(path)
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("VS Code launch failed: {e}"))?;

    Ok(())
}

fn open_folder(path: &Path) -> Result<(), String> {
    verify_directory(path)?;

    println!("  [Folder] {}", path.display());

    open::that(path).map_err(|e| format!("Cannot open folder: {e}"))
}

fn open_website(url: &str) -> Result<(), String> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err(format!("Invalid website URL: {url}"));
    }

    println!("  [Web] {url}");

    open::that(url).map_err(|e| format!("Cannot open website: {e}"))
}

fn open_terminal(terminal: &Terminal) -> Result<(), String> {
    verify_directory(&terminal.path)?;

    println!("  [Terminal] {}", terminal.path.display());

    #[cfg(target_os = "windows")]
    {
        let shell = terminal.shell.as_deref().unwrap_or("powershell");

        let executable = match shell.to_ascii_lowercase().as_str() {
            "powershell" => "powershell.exe",
            "pwsh" => "pwsh.exe",
            "cmd" => "cmd.exe",
            "bash" => "bash.exe",
            _ => return Err(format!("Unsupported Windows shell: {shell}")),
        };

        let mut command = Command::new("cmd");
        command.args(["/C", "start", "", executable]);

        match shell.to_ascii_lowercase().as_str() {
            "powershell" | "pwsh" => {
                command.arg("-NoExit");
            }
            "/none" => {}
            _ => {}
        }

        command
            .current_dir(&terminal.path)
            .spawn()
            .map_err(|e| format!("Terminal launch failed: {e}"))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Try commonly installed Linux terminal emulators.
        let path = &terminal.path;
        let path_str = path.to_string_lossy();

        let candidates: Vec<(&str, Vec<String>)> = vec![
            (
                "gnome-terminal",
                vec![format!("--working-directory={path_str}")],
            ),
            ("konsole", vec!["--workdir".into(), path_str.to_string()]),
            (
                "xfce4-terminal",
                vec![format!("--working-directory={path_str}")],
            ),
            ("xterm", vec!["-e".into(), "sh".into()]),
        ];

        let mut started = false;

        for (program, args) in candidates {
            if Command::new(program)
                .args(&args)
                .current_dir(path)
                .spawn()
                .is_ok()
            {
                started = true;
                break;
            }
        }

        if !started {
            return Err("No supported terminal emulator found".into());
        }
    }

    Ok(())
}

fn run_custom_command(item: &CustomCommand) -> Result<(), String> {
    if item.program.trim().is_empty() {
        return Err("Custom command program cannot be empty".into());
    }

    println!("  [Command] {}", item.program);

    let mut cmd = Command::new(&item.program);
    cmd.args(&item.args);

    if let Some(cwd) = &item.cwd {
        verify_directory(cwd)?;
        cmd.current_dir(cwd);
    }

    cmd.spawn()
        .map_err(|e| format!("Custom command failed: {e}"))?;

    Ok(())
}
