use colored::Colorize;
pub async fn install_package(pkg: Vec<&str>, appl: &crate::ApplInstance) {
    let time = std::time::Instant::now();
    let mut packages = Vec::new();
    for search in pkg {
        packages.extend(appl.clone().search_exact(search));
    }
    println!("{packages:?}");
    if packages.len() == 0 {
        println!("{} (Took {:?})", "No results found.".bold(), time.elapsed());
    } else {
        println!("\nFound in {:?}", time.elapsed());
        crate::table::install_search(packages.clone());
        let confirm = if crate::config::get_config_value("config", "prefer_md").unwrap() == "true" {
            crate::input::confirm_prompt_simple("Install these packages?")
        } else {
            crate::input::confirm_prompt("Install these packages?")
        };
        if confirm {
            for package in packages.iter() {
                println!("Installing {}", package.name);
                let _install = package.clone().install().await;
                if _install.is_err() {
                    println!(
                        "[{}] Failed to install {}",
                        "ERROR".red().bold(),
                        package.name
                    );
                    println!("Error caught: \n{}", _install.err().unwrap());
                }
            }
        } else {
            println!("Cancelled");
            std::process::exit(0);
        }
    }
}
pub async fn run_package(pkg: Vec<&str>, appl: &crate::ApplInstance) {
    let package = appl.clone().search_exact(pkg[0]);
    if package.is_empty() {
        println!("{}", "No results found.".bold());
    } else {
        let found = package[0].clone();
        found.run().await;
    }
}
pub async fn get_pkg_info(pkg: Vec<&str>, appl: &crate::ApplInstance) {
    let package = appl.clone().search_exact(pkg[0]);
    if package.is_empty() {
        println!("{}", "No results found.".bold());
    } else {
        let found = package[0].clone();
        println!(
            "{} {}",
            "Name         ::".bold(),
            found.name.bold().bright_red()
        );
        println!(
            "{} {}",
            "Version      ::".bold(),
            found.version.bold().green()
        );
        println!("{} {}", "Description  ::".bold(), found.desc.bold().white());
        println!("{} {}", "Repository   ::".bold(), found.repo.bold().cyan());
        println!(
            "{} {}",
            "Dependencies ::".bold(),
            found
                .depends
                .join(" | ")
                .trim_matches('"')
                .bold()
                .bright_magenta()
        );
        println!(
            "{} {}",
            "Author(s)    ::".bold(),
            found
                .authors
                .join(" | ")
                .trim_matches('"')
                .bold()
                .bright_blue()
        );
        println!(
            "{} {}",
            "Homepage     ::".bold(),
            found.homepage.bold().yellow()
        );
        println!(
            "{} {}",
            "License      ::".bold(),
            found.license.bold().magenta()
        );
    }
}
pub async fn search_package(pkg: Vec<&str>, appl: &crate::ApplInstance) {
    let mut packages = Vec::new();
    for search in pkg {
        packages.extend(appl.clone().search(search));
    }
    if packages.len() == 0 {
        println!("{}", "No results found.".bold());
    } else {
        crate::table::rough_search(packages.clone());
    }
}
