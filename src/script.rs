// Appl module for reading the scripts; then executing them.
// Released in the public domain under the Unlicense [https://Unlicense.org/]

#![allow(dead_code)]

use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::{PathBuf, Path}, collections::HashMap, str::FromStr, process::Command,
};

use sevenz_rust::decompress_file;
use colored::Colorize;
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
    fn extract_tar(archive: PathBuf, destination: PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::open(archive.to_str().unwrap())?;
        Ok(())
    }
    fn extract_7z(archive: PathBuf, destination: PathBuf) -> Result<(), Box<dyn Error>> {
        decompress_file(archive, destination).unwrap();
        Ok(())
    }
    pub fn extract_zip(source: &Path, dest: &Path) -> zip::result::ZipResult<()> {
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
}

pub fn syscmd(cmd: &str, args: Vec<&str>) {
    std::process::Command::new(cmd).args(args).output().expect("works");
}


/// Reads a file's build[] function, then tokenizes and executes it.
/// Accepts a String or a &str as input.
/// 
/// # Examples
/// ```
/// # use appl::script::read_build_script;
/// use appl::pkgutils::get_config;
/// let config = get_config();
/// read_build_script(format!("{config}vim.toml"));
/// ```
pub fn read_build_script<T: ToString>(file: T) {
    let mut defined_vars: HashMap<String, String> = HashMap::new();
    let mut global_variables: HashMap<String, String> = HashMap::new();

    global_variables.insert("@home".into(), std::env::var("HOME").unwrap());
    global_variables.insert("@version".into(), "0.6.2-alpha".into());

    let p = file.to_string();
    let path = Path::new(&p);
    let pkgname = path.file_stem().unwrap().to_str().unwrap();

    println!("{}{} {}", "=".blue(), ">".green(), pkgname.green().bold());

    global_variables.insert("@pkgdir".into(), get_app_folder(&get_app_name(pkgname)));
    
    let toml = read_toml(p); 
    let toml_keys: toml::Value = toml::from_str(&toml).unwrap(); // Deserialize the toml file into keys
    let build = get_build_func(toml_keys);

    for mut command in build {

        // Check to see if any parts of the command contain recognized variables.
        for i in 0..command.len() {
            if let Some(value) = defined_vars.keys().find(|&key| command[i].contains(key)) {
                let replacement = defined_vars.get(value).unwrap();
                command[i] = command[i].replace(value, replacement);
            }
            if let Some(value) = global_variables.keys().find(|&key| command[i].contains(key)) {
                let replacement = global_variables.get(value).unwrap();
                command[i] = command[i].replace(value, replacement);
            }
        }
        exec_command(&mut command, defined_vars.clone(), global_variables.clone());
        
    }
}

pub fn read_run_script<T>(file: T) where T: ToString {
    let defined_vars: HashMap<String, String> = HashMap::new();
    let mut global_variables: HashMap<String, String> = HashMap::new();

    global_variables.insert("@home".into(), std::env::var("HOME").unwrap());
    global_variables.insert("@version".into(), "0.6.2-alpha".into());

    let p = file.to_string();
    let path = Path::new(&p);
    let pkgname = path.file_stem().unwrap().to_str().unwrap();

    println!("{}{} {} {}", "=".blue(), ">".green(),"Running".green().bold(), pkgname.green().bold());

    global_variables.insert("@pkgdir".into(), get_app_folder(&get_app_name(pkgname)));
    
    let toml = read_toml(p); 
    let toml_keys: toml::Value = toml::from_str(&toml).unwrap(); // Deserialize the toml file into keys
    let build = get_build_func(toml_keys["run"].clone());

    for mut command in build {

        // Check to see if any parts of the command contain recognized variables.
        for i in 0..command.len() {
            if let Some(value) = defined_vars.keys().find(|&key| command[i].contains(key)) {
                let replacement = defined_vars.get(value).unwrap();
                command[i] = command[i].replace(value, replacement);
            }
            if let Some(value) = global_variables.keys().find(|&key| command[i].contains(key)) {
                let replacement = global_variables.get(value).unwrap();
                command[i] = command[i].replace(value, replacement);
            }
        }
        exec_command(&mut command, defined_vars.clone(), global_variables.clone());
        
    }
}

fn exec_command(command: &mut Vec<String>, mut defined_vars: HashMap<String,String>, global_vars: HashMap<String,String>) {
    match command[0].as_str() { // Match the command and execute it accordingly. TODO make this more secure and maybe async (may cause a race condition or panics)
            
        "print" => { // pretty self explanatory here man
            command.remove(0);
            println!("{:?}", command);
            
        },
        "bash" => { // Run a bash command in the current dir
            command.remove(0);
            Command::new("bash").arg("-c").args(command);
            
        },
        "define" => { // Define a varible
            defined_vars.insert(command[1].clone(), command[2].clone()); 
            
        },
        "make" => { // Executes CMake/Make in the specified directory. 
            let path = command[1].as_str();
            syscmd("make", vec![path]);
            
        },
        "clone" => { // Clones a git repository. Example: "clone vim/vim $PKGDIR/"
            let repo = format!("https://github.com/{}.git", command[1]);
            println!("Cloning repo {repo}");
            syscmd("git", vec!["clone", &repo]);
            
        },
        "extract" => { // Extract an archive. Format: ARCHIVE/DESTINATION/COMPRESSION TYPE
            match command[3].as_str() {
                "tar" => {CompressionTypes::extract_tar(command[1].clone().into(), command[2].clone().into());},
                "zip" => {CompressionTypes::extract_zip(Path::new(&command[1].clone()), Path::new(&command[2].clone()));},
                "7z                                              " => {CompressionTypes::extract_7z(command[1].clone().into(), command[2].clone().into());},       
                _ => {},
            }
            
        },
        "get-file" => { // Download a external file
            download_file(&command[1],  &command[2], command[3].green()).unwrap();
            
        }
        "load" => { // Load a different build script. Circular loops are possible
            read_build_script(command[1].clone());
            // Will allow for external scripts to run inside the script (say setting up lua, then neovim)
            
        },
        _ => {
            println!("Unrecognized command {}, skipping line.", command[0]);
        },
    }
}
pub fn get_build_func(key: toml::Value) -> Vec<Vec<String>> {
    let script: Vec<String> = key["build"] // Get the build() function of a build script
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();

    let script_split: Vec<Vec<String>> = script // Split the function into a list of commands (yes, a vector of vectors)
        .iter()
        .map(|command| command.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
    script_split
}

use std::io::Read;

use crate::pkgutils::{download_file, get_app_folder, get_app_name};

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
