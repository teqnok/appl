use std::{fs::{File, self}, fmt::Display, error::Error, path::PathBuf, process::Command, io::{self, Write}};

use colored::Colorize;

use crate::{utils::main::read_pkg_file, pkgutils::get_config};

pub fn syscmd(cmd: &str, arguments: Vec<&str>) {
    match Command::new(cmd)
        .args(arguments)
        .spawn() {
            Ok(e) => {},
            Err(e) => {println!("Caught error when executing command {cmd}: {e:#?}")}
        };

}


pub fn read_script(script: PathBuf) -> Result<bool, Box<dyn Error>>{
    println!("{} {} {}{}", "==>".green(), "Running".bold(), script.as_os_str().to_str().unwrap().bold(), "()".bold());
    let script: String = read_pkg_file(script.to_str().unwrap(), "_run/run.as")?;
    for command in script.lines() {
        
        let mut arguments: Vec<&str> = command.split_whitespace().collect();
        println!("arguments: {arguments:#?}");
        let cmd: &str = arguments.remove(0);
        match cmd {
            "make" => {syscmd("make", arguments)},
            "tar" => {
                arguments.insert(0, "-xf");
                syscmd("tar", arguments)
            },
            "print" => {for arg in arguments {print!("{arg} ")} println!()},
            "define" => {},
            "exec" => {
                arguments.insert(0, "-c");
                println!("command: {}", format!("bash {arguments:#?}"));
                syscmd("bash", arguments)
            }
            _ => {}
        }
    };
    Ok(true)
}
fn match_command(cmd: &str, args: Vec<&str>) {
    match cmd {
        "git" => {
            syscmd("git", args)
        }
        _ => {println!("Command {} not found, skipping...", cmd.yellow())}
    }
}