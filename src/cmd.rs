use clap::{Arg, ArgAction, ArgMatches, Command};
pub async fn builder(appl: crate::ApplInstance) {
    let matches = Command::new("appl")
        .about("Portable package manager")
        .version("0.6.2-alpha")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("teqnok [teqnok@proton.me]")
        .subcommand(
            Command::new("install")
                .about("Install a package from the loaded repository")
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
                .arg_required_else_help(true)
                .long_about(
                    "May have to infer what to open a file with, and may not work with mods.",
                )
                .arg(Arg::new("package").index(1).action(ArgAction::Set)),
        )
        .subcommand(Command::new("setup").about("Enter the applsetup tool"))
        // Query subcommand
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .arg_required_else_help(true)
                .about("Show information about the given package")
                .subcommand(
                    Command::new("info")
                        .short_flag('i')
                        .arg_required_else_help(true)
                        .about("View information about a package")
                        .arg(Arg::new("package").index(1).action(ArgAction::Set)),
                )
                .subcommand(
                    Command::new("build")
                        .about("Run a package's build function.")
                        .arg(Arg::new("package").index(1).action(ArgAction::Set)),
                )
                .subcommand(
                    Command::new("search")
                        .short_flag('s')
                        .arg_required_else_help(true)
                        .about("Search for a package")
                        .arg(Arg::new("package").index(1).action(ArgAction::Set)),
                )
                .arg(Arg::new("package").index(1).action(ArgAction::Set)),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("install", install_matches)) => {
            let args = collect_input(install_matches);
            crate::cli::install_package(args, &appl).await;
        }
        Some(("run", run_matches)) => {
            let args = collect_input(run_matches);
            crate::cli::run_package(args, &appl).await;
        },
        Some(("query", q)) => match q.subcommand() {
            Some(("search", search_matches)) => {
                let args = collect_input(search_matches);
                crate::cli::search_package(args, &appl).await;
            }
            Some(("info", info_matches)) => {
                let args = collect_input(info_matches);
                crate::cli::get_pkg_info(args, &appl).await;
            }
            _ => {
                println!("Query subcommand not found.")
            }
        },
        _ => {
            println!("Command not found.")
        }
    }
}
fn collect_input(matches: &ArgMatches) -> Vec<&str> {
    let packages: Vec<&str> = matches
        .get_many::<String>("package")
        .expect("is present")
        .map(|s| s.as_str())
        .collect();
    packages
}
