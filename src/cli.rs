pub fn install_package(pkg: &str, appl: crate::ApplInstance) {
    let mut avg = Vec::new();
    let time = std::time::Instant::now();
    let packages = appl.clone().search_exact(pkg);
    avg.push(time.elapsed().as_micros());

    crate::table::render_package_table(packages.clone());
    println!("\nFound in {:?}", time.elapsed());
    let confirm = if crate::config::get_config_value("config", "prefer_md").unwrap() == "true" {
        crate::input::confirm_prompt_simple("Install these packages?")
    } else {
        crate::input::confirm_prompt("Install these packages?")
    };
    if confirm {
        for package in packages.iter() {
            package.install();
        }
    } else {
        println!("Cancelled");
        std::process::exit(0);
    }
}
