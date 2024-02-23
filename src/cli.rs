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
                let _install = package.clone().install();
                println!("{:?}", _install.await);
            }
        } else {
            println!("Cancelled");
            std::process::exit(0);
        }
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
