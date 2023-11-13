// Appl module for reading the scripts; then executing them.
// Released in the public domain under the Unlicense [https://Unlicense.org/]
#![allow(dead_code)]
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::{PathBuf, Path}, collections::HashMap, str::FromStr,
};
use sevenz_rust::decompress_file;
use colored::Colorize;
use tar::*;
use zip::ZipArchive;
#[allow(non_camel_case_types)]
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
    pub fn extract_7z(archive: PathBuf, destination: PathBuf) -> Result<(), Box<dyn Error>> {
        decompress_file(archive, destination).unwrap();
        Ok(())
    }
    fn extract_zip(source: &Path, dest: &Path) -> zip::result::ZipResult<()> {
        let file = File::open(source)?;
        let mut zip = ZipArchive::new(file)?;
    
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let outpath = dest.join(file.name());
    
            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        Ok(())
    }
    // Wrapper function around all of the extract functions
    // Takes input like: "lua.zip", matches the file extension, then runs the appropriate function.
    fn extract(archive: &str, dest: PathBuf) {
        let path = Path::new(archive);
        let ext = Path::new(archive).extension().and_then(|os_str| os_str.to_str());
        match ext.unwrap() {
            "zip" => {
                CompressionTypes::extract_zip(path, &dest).unwrap();
            },
            "7z" => {
                CompressionTypes::extract_7z(path.to_path_buf(), dest).unwrap();
            },
            ".gz" => {
                CompressionTypes::extract_tar(path.to_path_buf(), dest).unwrap();
            },

            _ => {
                println!("Tried to extract a file, but the extension was not valid!");
            }
        }
    }
}

pub fn syscmd(cmd: &str, args: Vec<&str>) {
    std::process::Command::new(cmd).args(args).output().expect("works");
}


// Read && execute the build function of a package. Supports &str / String as inputs.
// TODO introduce variables for the script to use
// TODO work with repositories
pub fn read_build_script<T: ToString>(file: T) {
    let mut defined_vars: HashMap<String, String> = HashMap::new();
    let mut global_variables: HashMap<String, String> = HashMap::new();
    global_variables.insert("@home".into(), std::env::var("HOME").unwrap());
    global_variables.insert("@pkgdir".into(), "d".into());
    global_variables.insert("@version".into(), "0.6.2-alpha".into());
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
                if global_variables.contains_key(&command[1]) {
                    command[1] = global_variables.get(&command[1]).unwrap().to_string();
                }
                if defined_vars.contains_key(&command[1]) {
                    command[1] = defined_vars.get(&command[1]).unwrap().to_string();
                } 
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
                CompressionTypes::extract(command[1].as_str(), PathBuf::from_str(command[2].as_str()).unwrap());
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
