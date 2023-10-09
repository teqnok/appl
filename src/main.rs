//------------------------------------------------------------------------------
//              Advanced Portable Package Loader
//           Developed by teqnok [teqnok@proton.me] 
//      Released in the public domain via the Unlicense
//------------------------------------------------------------------------------

use appl::{clear, Package, Branch, Architecture, install_package, read_build_script, collect_input, list_packages};
use clap::{Arg, ArgAction, Command};
use std::{fs, path::Path};
use whoami;

use crate::setup::setup;

mod prompt;
mod setup; 
fn main() {
    // DEBUG CODE ----- NOT FOR PROD BUILDS
    // END DEBUG
    let current_user: String = whoami::username();
    // These lines check for a config file that doesnt exist, will fix. TODO 
    let config_path: String = format!("/home/{current_user}/.config/appl/");
    match Path::new(&config_path).try_exists() {
        Ok(true) => {},
        Ok(false) => {
            setup();
        },
        Err(e) => {
            println!("Caught exception when looking for config file: {:?}", e)
        }
    }

    // Define the `appl` command
    let matches = Command::new("appl")
        .about("Portable Package Manager")
        .version("0.2.4-alpha")
        .subcommand_required(false)
        // This should be false for dev and true for prod
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
                let packages = collect_input(install_matches);
                
                println!("Searching for {}...", packages[0]);
                read_build_script(packages[0]).expect(":)");
                
            },
            Some(("query", query_matches)) => {
                let packages = collect_input(query_matches);
                println!("Searching for {:?}", packages);
            },
            Some(("remove", remove_matches)) => {
                let packages = collect_input(remove_matches);
                println!("Uninstalling packages {:?}", packages)
            },
            Some(("list", _list_matches)) => {
                list_packages().expect(":)");
            }
            _ => todo!(""),
        }
    }
