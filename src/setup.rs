// Setup module for appl
//
//

use std::fs;

use colored::Colorize;
use std::process::Command;
/// Checks to see if a program is installed.
/// Currently only used in the setup process.
fn is_installed(cmd: &str) -> bool {
    let output = Command::new(cmd)
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    output.status.success()
}
/// Creates config directories and checks to see what modules can be loaded.
pub fn setup() -> u32 {
    println!("[1/6] Creating config folders");
    make_configs();
    println!("{}", "[2/6] Checking if git is installed...".green());
    if is_installed("git") {
        println!("git --version successful, enabling git command")
    } else {
        println!("git not found, build scripts will not be able to use the 'clone' command.");
    }

    println!("{}", "[3/6] Checking if make is installed...".green());
    if is_installed("make") {
        println!("make --version successful, enabling make command")
    } else {
        println!("make not found, build scripts will not be able to use the 'make' command.");
    }
    println!("{}", "[4/6] Checking if bash is installed".green());
    if is_installed("bash") {
        println!("bash --version successful, enabling bash command")
    } else {
        println!("Bash not found, build scripts will not be able to use the 'bash' command.");
    }
    println!();

    println!();
    0
}
#[cfg(not(windows))]
fn make_configs() {
    let uname = whoami::username();

    fs::create_dir(format!("/home/{uname}/.config/appl"));
    fs::create_dir(format!("/home/{uname}/Apps/"));
}
#[cfg(windows)]
fn make_configs() {
    let path = std::env::var("HOME").unwrap();

    fs::create_dir(format!("{path}/AppData/Local/appl/"));
    fs::create_dir(format!("{path}/Apps/"));
}
