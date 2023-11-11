//------------------------------------------------------------------------------------------
//
//              Advanced Portable Package Loader (Library)
//           Available in the public domain via the Unlicense
//
//------------------------------------------------------------------------------------------
use checksums::{hash_file, Algorithm};
use colored::{ColoredString, Colorize};
use pkgutils::{download_file, read_repos};
use prompt::{int_input, prompt_input, select_prompt, select_prompt_string};
use std::fmt::{self, Display};
use std::io::Read;
use std::process::Command;
use std::time::Instant;
use std::{fs::File, path::Path};
mod pkgutils;
// ----------------------------
// Define supported architectures and branches for a package
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Architecture {
    X64,
    X32,
    arm64,
    any,
}
impl Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::X32 => write!(f, "{}", "X32".blue()),
            Architecture::X64 => write!(f, "{}", "X64".green()),
            Architecture::arm64 => write!(f, "{}", "arm64".red()),
            Architecture::any => write!(f, "{}", "any".bright_blue()),
        }
    }
}
pub enum ApplError {
    InvalidArchitecture,
    InvalidBranch,
}
impl Architecture {
    pub fn from_str(string: &str) -> Architecture {
        let mut arch = Architecture::X64;
        match string {
            "X32" => arch = Architecture::X32,
            "X64" => arch = Architecture::X64,
            "arm64" => arch = Architecture::arm64,
            "any" => arch = Architecture::any,
            _ => println!("{}", "TOML Script does not have a valid architecture set. Please ensure the script is valid.".yellow()),
        }
        arch
    }
}
#[derive(Debug)]
pub enum Branch {
    dev,
    prod,
    git,
    beta,
    nightly,
}

pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}
impl Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Branch::dev => write!(f, "{}", "dev".purple()),
            Branch::prod => write!(f, "{}", "prod".bright_green()),
            Branch::git => write!(f, "{}", "git".bright_blue()),
            Branch::beta => write!(f, "{}", "beta".bright_cyan()),
            Branch::nightly => write!(f, "{}", "nightly".bright_magenta()),
        }
    }
}
impl Branch {
    pub fn from_str(string: &str) -> Branch {
        let mut branch = Branch::prod;
        match string {
            "dev" => branch = Branch::dev,
            "prod" => branch = Branch::prod,
            "git" => branch = Branch::git,
            "beta" => branch = Branch::beta,
            "nightly" => branch = Branch::nightly,
            _ => println!(
                "{}",
                "TOML Script does not have a valid branch set. Please ensure the script is valid."
                    .yellow()
            ),
        }
        branch
    }
}

pub struct Package {
    arch: Architecture,
    branch: Branch,
    name: String,
    version: String,
    url: String,
    dependencies: Vec<String>,
    download_size: i64,
    install_size: i64,
}
impl Package {
    pub fn new(
        arch_input: Architecture,
        branch_input: Branch,
        name_input: String,
        version_input: String,
        url_input: String,
        dep_input: Vec<String>,
        download_input: i64,
        install_input: i64,
    ) -> Package {
        Package {
            arch: arch_input,
            branch: branch_input,
            name: name_input,
            version: version_input,
            url: url_input,
            dependencies: dep_input,
            download_size: download_input,
            install_size: install_input,
        }
    }
}
//----------------------------
// Clear terminal
pub fn clear() {
    assert!( std::process::Command::new("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success() );
}

// Download file (terrible, not even async, depends on curl, super insecure)
// pub fn get_url(url: String, destination: String) {
//     assert!(Command::new("curl")
//         .arg(url)
//         .arg("-O")
//         .arg(destination)
//         .status()
//         .unwrap()
//         .success())
// }

use crate::prompt::confirm_prompt_custom;
use crate::script::read_build_script;
use clap::ArgMatches;
use walkdir::WalkDir;
mod prompt;
mod script;
use std::collections::HashMap;
// Root command for installing packages,
pub fn install_package(input: Vec<&str>) -> std::io::Result<()> {
    let time = Instant::now();
    // DEFINE VARIABLES
    let current_user: String = whoami::username(); //
    let config_path: String = format!("/home/{current_user}/.config/appl/"); //

    // METADATA VECTORS
    let mut packages_to_install: Vec<String> = vec![]; //
    let mut packages: Vec<Package> = vec![];

    let mut found_terms: HashMap<&str, bool> = HashMap::new();
    let mut not_found_terms = Vec::new();
    let mut scripts = Vec::new();
    //----------------------------------------------------a
    // ADD VARIABLES TO INSTALL LIST
    for &path in &input {
        found_terms.insert(path, false);
    }
    
    for entry in WalkDir::new(&config_path) {
        let entry = entry.unwrap();
        let entry_str = entry.path();
        let entry_path = entry_str.with_extension("");
        if let Some(file_name) = entry_path.file_name() {
            let file_name_str = file_name.to_str().unwrap();
            if let Some(found) = found_terms.get_mut(file_name_str) {
                *found = true;
                let pkg_path = entry_str.to_str().unwrap().to_string();
                packages_to_install.push(pkg_path);
            }
        }
    }
    
    for (path, found) in &found_terms {
        if !found {
            not_found_terms.push(path.to_string());
        }
    }

    for pkg in not_found_terms {
        println!("{} {}. Skipping..", "Could not find result".red(), pkg.yellow())
    }

    if packages_to_install.is_empty() {
        println!(
            "{}",
            "Could not find any packages matching the search terms.".yellow()
        );
    } else {
        println!("Resolving dependencies... \n");

        // Add packages to a vector, to print a line for each package
        for package in packages_to_install {
            println!(
                "{} Build script found at: {}",
                "->".purple(),
                package.bright_blue().bold()
            );
            let toml_read = read_toml(package.clone().into());
            let toml_keys: toml::Value = toml::from_str(&toml_read).unwrap();
            packages.push(Package::new(
                Architecture::from_str(toml_keys["arch"].as_str().unwrap()),
                Branch::from_str(toml_keys["branch"].as_str().unwrap()),
                toml_keys["name"].to_string(),
                toml_keys["version"].to_string(),
                toml_keys["url"].to_string(),
                toml_keys["dependencies"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|str| str.to_string())
                    .collect(),
                toml_keys
                    .get("download_size")
                    .unwrap()
                    .as_integer()
                    .unwrap(),
                toml_keys.get("install_size").unwrap().as_integer().unwrap(),
            ));
            scripts.push(package);
        }
        print!("\n");

        println!("Packages to install: \n \t");
        let mut download_size: i64 = 0;
        let mut install_size: i64 = 0;
        let mut is_installed: String = "".into();

        for package in &packages {
            let path = format!(
                "/home/{}/Apps/{}",
                current_user,
                package.name.trim_matches('"')
            );
            match Path::new(&path).try_exists() {
                Ok(true) => is_installed = "[Installed]".bright_cyan().to_string(),
                Ok(false) => {}
                Err(e) => println!(
                    "Caught error {} when checking to see if a package was installed!",
                    e
                ),
            }
            let package_version = package.version.trim_matches('"').purple().bold();
            if package.dependencies.is_empty() {
                println!(
                    "{}{} {}-{}:{} [{}] {} \n",
                    "=".blue(),
                    ">".green(),
                    package.name.to_string().trim_matches('"').purple().bold(),
                    format!("{package_version}"),
                    package.branch,
                    package.arch,
                    is_installed,
                )
            } else {
                println!(
                    "{}{} {}-{}:{} [{}] {} \n{}{} (requires packages {}) \n",
                    "=".blue(),
                    ">".green(),
                    package.name.to_string().trim_matches('"').purple().bold(),
                    format!("{package_version}"),
                    package.branch,
                    package.arch,
                    is_installed,
                    "==".blue(),
                    ">".green(),
                    package.dependencies.join(" "),
                );
            }
            download_size += package.download_size;
            install_size += package.install_size;
        }
        // Convert the vector of Strings (ex ["3","4"]) to a vector of `i32`s (ex [3,4]), then add them (ex 7)
        println!(
            "Download size: {} MB \t Install size: {} MB [Took {:?}]",
            download_size.to_string().green().bold(),
            install_size.to_string().blue().bold(),
            time.elapsed()
        );

        let confirm_package_install: Result<bool, std::io::Error> =
            confirm_prompt_custom(String::from("Install these packages?"));
        match confirm_package_install {
            Ok(true) => {
                println!("[1/5] Downloading packages");
                for package in packages {
                    get_source(package.url.to_string(), package.name.to_string()).unwrap();
                }
                println!("[2/5] Running build scripts");
                for script in scripts {
                    read_build_script(script);
                }
                println!("[3/5] Verifying checksums");
                println!("[4/5] Running post-install modules");
                println!("[5/5] Creating .desktop files and adding to $PATH");
            }
            Ok(false) => println!(
                "{}",
                "Canceled operation".yellow()
            ),
            Err(e) => eprintln!("Caught exception {} when registering confirm prompt.", e),
        }
        
    }
    Ok(())
}

pub async fn verify_checksums(input: Vec<&str>, algorithm: Algorithm) -> std::io::Result<()> {
    let file = Path::new("/home/teqnok/.config/lvim/config.lua");
    let algo = Algorithm::SHA2256;
    println!("{}", hash_file(file, algo));
    Ok(())
}

pub async fn remove_package(package: Package) {
    let package_path: String = format!("/home/{}/Apps/{}", whoami::username(), package.name);
    let confirm_package_removal = confirm_prompt_custom("Remove these packages?".into());
}

use std::io::Error;
// Sub-function of read_build_script that executes the script part of the scripts. (wow!)
fn get_source(url_location: String, package_name: String) -> Result<(), Error> {
    let username = whoami::username();
    let package_name = package_name.trim_matches('"');
    let package_dir = format!("/home/{username}/Apps/{}", package_name);
    let path = Path::new(&package_dir);
    match path.try_exists() {
        Ok(true) => {
            println!(
                "{}",
                "Package already has a directory. Installation may fail.".red()
            );
        }
        Ok(false) => {
            assert!(Command::new("mkdir")
                .arg(package_dir.clone().trim_matches('"'))
                .status()
                .unwrap()
                .success());
        }
        Err(e) => eprintln!("Encountered exception {} while checking directory", e),
    };

    download_file(
        url_location.as_str(),
        &format!("{package_dir}/tmp/{package_name}"),
        package_name.green().bold(),
    )
    .unwrap();
    Ok(())
}

// Collect input from 'clap' and return a vector. Not related to anything else.
pub fn collect_input(matches: &ArgMatches) -> Vec<&str> {
    let packages: Vec<&str> = matches
        .get_many::<String>("package")
        .expect("is present")
        .map(|s| s.as_str())
        .collect();
    packages
}
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

pub fn new_package() {
    let new_name = prompt_input("What is the name of the package?".green());
    let new_branch = select_prompt(
        vec!["prod", "git", "nightly", "beta", "dev"],
        "What is the development branch of this package?".into(),
    );
    let new_arch = select_prompt(
        vec!["X64", "X32", "arm64", "any"],
        "What is this packages architecture?".into(),
    );
    let new_version = prompt_input("What is the version?".green().bold());
    let new_url = prompt_input(
        "Where is the projects source code? Input a download link."
            .green()
            .bold(),
    );
    let new_dsize = int_input(
        "What is the package's size to download (MiB)?"
            .bold()
            .green(),
    );
    let new_isize = int_input(
        "What is the package's size when installed (MiB)?"
            .bold()
            .green(),
    );
    let repo = select_prompt_string(
        read_repos().unwrap(),
        "Which repository should this package belong in?".into(),
    );
    let new_branch_u = new_branch.unwrap();
    let new_arch_u = new_arch.unwrap();
    let toml_string = format!(
        "
    name = {new_name} 
    branch = {new_branch_u} 
    arch = {new_arch_u} 
    version = {new_version} 
    url = {new_url} 
    download_size = {new_dsize} 
    install_size = {new_isize} 
    build = [
        'Put a build script here',
        'Then publish this file.'
    ]
    "
    );
    println!("{toml_string}");
    let repo_print = repo.unwrap();
    let write_confirm = confirm_prompt_custom(format!(
        "Write this package to '~/.config/appl/{repo_print}/{new_name}.toml'?"
    ))
    .unwrap();
    if write_confirm {
    } else {
        println!("{}", "Cancelling".red())
    }
}