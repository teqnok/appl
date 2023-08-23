use clap::{Arg, ArgAction, Command, error::ErrorKind};
use dialoguer::Confirm;
use indicatif::{ProgressBar, DecimalBytes};

fn main() {
    let matches = Command::new("tpl")
        .about("Fantasy package manager")
        .version("0.0.1-dev")
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
            Command::new("remove")
            .about("Uninstall a package")
            .arg(
                Arg::new("package").index(1).action(ArgAction::Set)
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
    
    
    }