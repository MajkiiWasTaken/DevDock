/************************************************
* File: output.rs
* Author: Michal Švrček
*
* DevDock colored terminal output
*
* ver. 0.4.1
*************************************************/

use colored::Colorize;

pub fn banner() {
    println!();
    println!(
        "{}",
        format!("◆ DevDock v{}", env!("CARGO_PKG_VERSION"))
            .bright_blue()
            .bold()
    );
    println!("{}", "────────────────────────────────────────".dimmed());
}

pub fn heading(message: &str) {
    println!();
    println!("{}", message.cyan().bold());
}

pub fn info(message: &str) {
    println!("{} {}", "[INFO]".bright_blue().bold(), message);
}

pub fn step(message: &str) {
    println!("{} {}", "[STEP]".yellow().bold(), message);
}

pub fn success(message: &str) {
    println!("{} {}", "[ OK ]".green().bold(), message.green());
}

pub fn warning(message: &str) {
    println!("{} {}", "[WARN]".yellow().bold(), message);
}

pub fn error(message: &str) {
    eprintln!("{} {}", "[ERROR]".red().bold(), message.red());
}

pub fn label(name: &str, value: &str) {
    println!("  {:<18} {}", name.bright_black(), value.white());
}

pub fn item(kind: &str, value: &str) {
    println!("  {} {}", format!("[{kind}]").bright_cyan().bold(), value);
}

pub fn session(name: &str) {
    println!("  {} {}", "●".green(), name.bright_white().bold());
}
