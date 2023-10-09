// Library functions for appl
// This API aims to provide the following:
//  - Methods and templates for creating a package 
//

use std::path::Path;
use std::process::Command;
//----------------------------
// Define supported architectures and branches for a package 
pub enum Architecture {
    X64,
    X32,
    arm64
}
#[derive(Debug)]
pub enum Branch {
    dev,
    prod,
    git,
    beta,
    nightly
}
pub struct Package {
    arch: Architecture,
    branch: Branch,
    name: String,
}
impl Package {
    pub fn new(arch_input: Architecture, branch_input: Branch, name_input: String) -> Package {
        return Package { arch: arch_input, branch: branch_input, name: name_input }
    }
}
//----------------------------
// Clear terminal
pub fn clear() {
    Package::new(Architecture::X64, Branch::dev, String::from("vim"));
    assert!( std::process::Command::new("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success() );
}

// Download file
pub fn get_url(url: String, destination: String) {
    assert!( 
        Command::new("curl")
        .arg(url)
        .arg("-O")
        .arg(destination)
        .status()
        .unwrap()
        .success()
    )
} 

// Function defining the process for installing a package. Only edit for major changes.
pub fn install_package(package: Package) {
    println!("Searching for {}", package.name);
    //TODO Implement searching for packages !!!!
    println!("Found match: {}@{:#?}", package.name, package.branch);
}

use std::fs::{read_to_string, self};

use clap::ArgMatches;

pub fn read_build_script(input: &str)-> std::io::Result<()> {

    let current_user: String = whoami::username();
    let config_path: String = format!("/home/{current_user}/.config/appl/");
    let input_path: String = format!("{config_path}{input}");

    let entries = fs::read_dir(config_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    for entry in entries {
        let entry_str: String = entry.to_string_lossy().to_string();
        
        if entry_str == input_path {
            println!("Found exact match: {:#?}", entry_str);
            let lines: Vec<_> = read_to_string(&input_path) 
                .unwrap()  // panic on possible file-reading errors
                .lines()  // split the string into an iterator of string slices
                .map(String::from)  // make each slice into a string
                .collect();  // gather them together into a vector

            for line in lines.iter() {
                assert!( 
                    Command::new("bash")
                    .arg("-c")
                    .arg(line)
                    .status()
                    .unwrap()
                    .success()
                )
            }
        }
    }
    println!("Could not find package. Are you using the right registry?");
    

    Ok(())

}

pub fn collect_input(matches: &ArgMatches) -> Vec<&str> {
    let packages: Vec<&str> = matches
    .get_many::<String>("package")
    .expect("is present")
    .map(|s| s.as_str())
    .collect();
    packages
}

pub fn list_packages() -> std::io::Result<()> {
    let current_user: String = whoami::username();
    let config_path: String = format!("/home/{current_user}/.config/appl/");

    let entries = fs::read_dir(config_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    for entry in entries {

        let file_name = entry.file_name().and_then(|s| s.to_str());
        match file_name {
            Some(name) => println!("{:#?}", name),
            None => println!("Could not get file name")
        }

    }
    Ok(())
}

pub fn check_for_package(input: &str) -> std::io::Result<()> {

    let current_user: String = whoami::username();
    let config_path: String = format!("/home/{current_user}/Apps");
    let input_path: String = format!("{config_path}{input}");

    let entries = fs::read_dir(config_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    for entry in entries {
        let entry_str: String = entry.to_string_lossy().to_string();
        
    }
    println!("Could not find package. Are you using the right registry?");
    

    Ok(())

}