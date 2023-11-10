// Setup module for appl
// Creates config file and PIN
// Provides preset trees to use

use std::fs;

use crate::prompt::{self, select_prompt};
use appl::clear;
pub fn setup() -> u32 {
    let current_user: String = whoami::username();

    // These lines create a config file, which will stop setup. Uncomment them for production builds
    let paths = vec![
        format!("/home/{current_user}/.config/appl"),
        format!("/home/{current_user}/Apps"),
    ];
    for path in paths {
        let _ = fs::create_dir(&path);
    }
    clear();
    println!("Found recommended architecture of {:?}", whoami::arch());
    let confirm_setup =
        prompt::confirm_prompt_custom(String::from("Would you like to enter setup?"));
    match confirm_setup {
        Ok(true) => {}
        Ok(false) => return 1,
        Err(e) => {
            println!("Caught error {:?}", e)
        }
    }
    let input = prompt::create_password("Create a PIN to install packages", "Repeat the PIN");
    let confirm = prompt::confirm_prompt_custom(String::from(
        "Would you like to use a predefined package list?",
    ));
    match confirm {
        Ok(val) => {
            if val {
                select_prompt(
                    vec!["ROMs", "Development Tools", "MC Mods", "Complete"],
                    String::from("Select a package base"),
                )
                .expect(":)");
            }
            if !val {
                println!("no")
            }
        }
        Err(_) => todo!(),
    }
    0
}
