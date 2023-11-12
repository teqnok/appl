// Appl module for reading the scripts; then executing them.
// Released in the public domain under the Unlicense [https://Unlicense.org/]
#![allow(dead_code)]
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::{PathBuf, Path}, collections::HashMap,
};
use colored::Colorize;
use tar::*;
#[allow(non_camel_case_types)]
const VARIABLES: [&'static str; 5] = ["@pkgsrc", "@pkgdir", "@home", "", ""];
pub enum CompressionTypes {
    Zip,
    Tar,
    SevenZip,
    Rar,
    GZip,
    BZip2,
}

impl CompressionTypes {
    pub fn extract_tar(archive: PathBuf, destination: PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::open(archive.to_str().unwrap())?;
        let mut archive = Archive::new(BufReader::new(file));
        archive.unpack(destination)?;
        Ok(())
    }
    pub fn extract_zip() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn syscmd(cmd: &str, args: Vec<&str>) {
    std::process::Command::new(cmd).args(args).output().expect("works");
}

pub enum Variables {
    PKGDIR,
    PKGSRC,
}

// Read && execute the build function of a package. Supports &str, Vec<&' _Iterator>, and String as inputs.
// TODO introduce variables for the script to use
// TODO work with repositories
pub fn read_build_script<T: ToString>(file: T) {
    let mut defined_vars: HashMap<String, String> = HashMap::new();
    let p = file.to_string();
    let path = Path::new(&p);
    let pkgname = path.file_stem().unwrap().to_str().unwrap();
    println!("{}{} {}", "=".blue(), ">".green(), pkgname.green().bold());

    let toml = read_toml(p); 
    let toml_keys: toml::Value = toml::from_str(&toml).unwrap(); // Deserialize the toml file into keys
    let script: Vec<String> = toml_keys["build"] // Get the build() function of a build script
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();

    let script_split: Vec<Vec<String>> = script // Split the function into a list of commands (yes, a vector of vectors)
        .iter()
        .map(|command| command.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    for mut command in script_split {
        
        match command[0].as_str() { // Match the command and execute it accordingly. TODO make this more secure and maybe async (may cause a race condition or panics)
            "print" => { // pretty self explanatory here man
                if defined_vars.contains_key(&command[1]) {
                    command[1] = defined_vars.get(&command[1]).unwrap().to_string();
                };
                println!("{}", command[1]);
                continue
            },
            "define" => {
                defined_vars.insert(command[1].clone(), command[2].clone()); 
            },
            "make" => { // Executes CMake/Make in the specified directory. 
                let path = command[1].as_str();
                syscmd("make", vec![path]);
                continue
            },
            "clone" => { // Clones a git repository. Example: "clone vim/vim $PKGDIR/"
                let repo = format!("https://github.com/{}.git", command[1]);
                println!("Cloning repo {repo}");
                syscmd("git", vec!["clone", &repo]);
                continue
            },
            "extract" => {
                // TODO implement function for extracting tar.gz/zip/7z/rar
                // Halfway done, just need 7z and rar.
                continue
            },
            "get-file" => {
                download_file(&command[1],  &command[2], command[3].green()).unwrap();
            }
            "load" => {
                // Will allow for external scripts to run inside the script (say setting up lua, then neovim)
                continue
            },
            _ => {},
        }
    }
}



use std::io::Read;

use crate::pkgutils::download_file;

fn read_toml(file: String) -> String {
    let path = Path::new(&file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Could not open File {}", e),
    };
    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    string
}
