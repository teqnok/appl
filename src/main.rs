use clap::{Arg, ArgAction, Command, error::ErrorKind};
use dialoguer::Confirm;
use indicatif::{ProgressBar, DecimalBytes};

macro_rules! getpkg {
    () => {
        
    };
}

fn main() {
    let matches = Command::new("appl")
        .about("AppImage Package Manager")
        .version("0.2.4-alpha")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("teqnok")

        .subcommand(
            Command::new("install")
            .about("Install a package from the loaded database")
            .arg(
                Arg::new("package").index(1).action(ArgAction::Set)
            )
        )
        .subcommand(
            Command::new("verify")
            .about("Verify checksums of installed AppImages")
            .arg (
                Arg::new("type").index(1).action(ArgAction::Set)
            )
            .arg (
                Arg::new("package").index(2).action(ArgAction::Set)
            )
        )

        .subcommand(
            Command::new("reqdel")
            .about("Request deletion of data")
            .arg ( 
                Arg::new("uuid").index(1).action(ArgAction::Set)
            )
        )

        .subcommand(
            Command::new("config")
            .arg (
                Arg::new("option").index(1).action(ArgAction::Set)
            )
            .arg (
                Arg::new("value").index(2).action(ArgAction::Set)
            )
        )

        .subcommand(
            Command::new("remove")
            .about("Uninstall a package")
            .arg(
                Arg::new("package").index(1).action(ArgAction::Set)
            )
        )
        .subcommand(
            Command::new("publish")
            .about("Request to add an AppImage to the registry")
            .long_about("When a package is submitted, various malware tests are run on the provided file.")
        )
        .subcommand(
            Command::new("list")
            .about("List all currently installed packages containing input characters. Basically, grep for your packagelist.")
            .arg(
                Arg::new("regex").index(1).action(ArgAction::Set)
            )
        )
        .subcommand(
            Command::new("discover")
            .about("Search the local database for packages")
            .arg(
                Arg::new("package").index(1).action(ArgAction::Set)
            )
        )
        // Query subcommand
        .subcommand(
            Command::new("query")
            .about("Show information about the given package")
            .arg(
                Arg::new("package").index(1).action(ArgAction::Set)
            )
        )

        .get_matches();
    
        
        
        match matches.subcommand() {
            Some(("install", install_matches)) => {
                let packages: Vec<_> = install_matches
                    .get_many::<String>("package")
                    .expect("is present")
                    .map(|s| s.as_str())
                    .collect();
                
                println!("Installing {:?}", install_matches);
                
            },
            Some(("query", query_matches)) => {
                let packages: Vec<_> = query_matches
                    .get_many::<String>("package")
                    .expect("is present")
                    .map(|s| s.as_str())
                    .collect();
                println!("Searching for {:?}", packages);
            },
            Some(("remove", remove_matches)) => {
                let packages: Vec<_> = remove_matches
                    .get_many::<String>("package")
                    .expect("is present")
                    .map(|s| s.as_str())
                    .collect();
                println!("Uninstalling packages {:?}", packages)
            }
            _ => todo!(""),
        }
    }
