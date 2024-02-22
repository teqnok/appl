use colored::Colorize;

pub fn setup() {
    println!("")
}

pub fn install() {
    let mut script_path: String = "~/.appl/scripts/".to_owned();
    let mut repo_path: String = "~/.appl/repos/".to_owned();
    let mut config_path: String = "~/.appl/config/".to_owned();
    let mut usefancy = true;
    loop {
        println!("Appl will use the following:\n");
        println!("Scripts: {script_path}");
        println!("Repo: {repo_path}");
        println!("Config: {config_path}");
        println!("Use fancy prompts: {}", usefancy);
        let selection = crate::input::select_prompt(
            "Select an option:",
            ["Proceed with setup", "Edit paths", "Cancel setup"].to_vec(),
        );
        match selection {
            "Proceed with setup" => break,
            "Edit paths" => {
                script_path = crate::input::input_prompt("New script path: ", "~/.appl/scripts/");
                repo_path = crate::input::input_prompt("New repo path: ", "~/.appl/repos/");
                config_path = crate::input::input_prompt("New config path: ", "~/.appl/config/");
                usefancy = crate::input::confirm_prompt("Use fancy prompts? ");
                println!("{}\n", "Config set".bold());
            }
            "Cancel setup" => {
                println!("Exiting");
                break;
            }
            _ => {
                break;
            }
        }
    }
}
