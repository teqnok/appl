use clap::{Arg, ArgAction, Command, error::ErrorKind};
use dialoguer::Confirm;
use indicatif::{ProgressBar, DecimalBytes};

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
    
    
    }