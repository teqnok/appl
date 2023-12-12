use colored::Colorize;
pub const HELP: &str = "";
pub fn print_help() -> String {
    println!("{}", "\nAdvanced Portable Package Loader".green());
    println!("──────────────────────────────── \n");

    println!("Syntax: appl [command] [flags] [argument] \n");
    println!("{}", "Commands\n".bold().bright_green());
    println!(
        "{}  Install a given package(s)",
        "install".underline()
    );
    println!("{} \t Remove a package(s)", "remove".underline());
    println!("{} \t Customize appl", "config".underline());
    println!(
        "{} \t Verify a package's checksums (SHA-256 or bcrypt)",
        "verify".underline()
    );
    println!("{} \t Open a installed package", "run".underline());
    println!(
        "{} \t Manage your packages (list, search)",
        "query".underline()
    );
    "done".into()
}
