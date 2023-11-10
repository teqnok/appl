use colored::Colorize;
pub const HELP: &str = "";
pub fn print_help() {
    println!("{}", "\nAdvanced Portable Package Loader".green());
    println!("{}", "──────────────────────────────── \n");

    println!("Syntax: appl [command] [flags] [argument] \n");
    println!("{}", "Commands\n".underline().bold().bright_green());
    println!(
        "{}  {}",
        "install".underline(),
        "Install a given package(s)"
    );
    println!("{} \t {}", "remove".underline(), "Remove a package(s)");
    println!("{} \t {}", "config".underline(), "Customize appl");
    println!(
        "{} \t {}",
        "verify".underline(),
        "Verify a package's checksums (SHA-256 or bcrypt)"
    );
    println!("{} \t {}", "run".underline(), "Open a installed package");
    println!(
        "{} \t {}",
        "query".underline(),
        "Manage your packages (list, search)"
    );
}
