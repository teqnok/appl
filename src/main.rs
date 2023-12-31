//------------------------------------------------------------------------------
//              Advanced Portable Package Loader
//           Developed by teqnok [teqnok@proton.me]
//      Released in the public domain via the Unlicense
// please dont use this :>
//------------------------------------------------------------------------------

use appl::collect_input;
use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use script::main::read_script;
use std::path::{Path, PathBuf}; 
use utils::main::get_pkg_info;

use crate::{pkgutils::generate_checksum, setup::setup};
pub mod pkgutils;
pub mod prompt;
pub mod script;
pub mod setup;
pub mod utils;
fn main() {
    read_script(PathBuf::from("/home/teqnok/Documents/vim.apkg"));
    let current_user: String = whoami::username(); 
    // These lines check for a config file that doesnt exist, will fix. TODO
    match Path::new(&format!("/home/{current_user}/.config/appl/")).try_exists() {
        Ok(true) => {}
        Ok(false) => {
            setup();
        }
        Err(e) => {
            println!("Caught exception when looking for config file: {e:?}")
        }
    }

    // Define the `appl` command
    let matches = Command::new("appl")
        .about("Portable package manager")
        .version("0.6.2-alpha")
        .subcommand_required(false)
        // This should be false for dev and true for prod
        .arg_required_else_help(false)
        .arg_required_else_help(true)
        .author("teqnok [teqnok@proton.me]")
        .subcommand(
            Command::new("install")
                .about("Install a package from the loaded database")
                .arg_required_else_help(true)
                .short_flag('i')
                .arg(
                    Arg::new("package")
                        .index(1)
                        .num_args(1..10)
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify checksums of installed packages")
                .arg(Arg::new("type").index(1).action(ArgAction::Set))
                .arg(Arg::new("package").index(2).action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("config")
                .arg(Arg::new("option").index(1).action(ArgAction::Set))
                .arg(Arg::new("value").index(2).action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("remove")
                .about("Uninstall a package")
                .arg(Arg::new("package").index(1).action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("run")
                .about("Execute/open a specified package")
                .long_about(
                    "May have to infer what to open a file with, and may not work with mods.",
                )
                .arg(Arg::new("")),
        )
        .subcommand(Command::new("setup").about("Enter the applsetup tool"))
        // Query subcommand
        .subcommand(
            Command::new("query")
                .about("Show information about the given package")
                .subcommand(
                    Command::new("gen_hash")
                        .about("Generate a SHA2-256 hash using the package's source")
                        .arg(Arg::new("package").index(1).action(ArgAction::Set)),
                )
                .subcommand(
                    Command::new("build")
                        .about("Run a package's build function.")
                        .arg(Arg::new("package").index(1).action(ArgAction::Set)),
                )
                .subcommand(
                    Command::new("search")
                        .about("Search for a package")
                        .arg(Arg::new("package").index(1).action(ArgAction::Set)),
                )
                .arg(Arg::new("package").index(1).action(ArgAction::Set)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", install_matches)) => {
            let packages = collect_input(install_matches);
            if packages.len() == 1 {
                println!("Searching for a package...")
            } else {
                println!("Searching for {} packages...", packages.len());
            }
            let _ = get_pkg_info(packages.clone());
            // let _ = install_package(packages.clone());
        }
        Some(("query", query_matches)) => match query_matches.subcommand() {
            Some(("gen_hash", hash_matches)) => {
                let matches = collect_input(hash_matches);
                generate_checksum(matches[0])
            }
            Some(("build", _build_matches)) => {}
            Some(("search", search_matches)) => {
                let m = collect_input(search_matches);
                let _ = get_pkg_info(m);
            }
            _ => {}
        },
        Some(("remove", remove_matches)) => {
            let packages = collect_input(remove_matches);
            println!("Uninstalling packages {:?}", packages[0].green())
        }
        Some(("list", _list_matches)) => {}
        Some(("setup", _setup_matches)) => {
            setup();
        }
        _ => {}
    }
}
