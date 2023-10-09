// Setup module for appl
// Creates config file and PIN
// Provides preset trees to use

use std::fs;

use appl::clear;
use crate::prompt::{self, select_prompt};
pub fn setup() -> u32 {
    let current_user: String = whoami::username();

    // These lines create a config file, which will stop setup. Uncomment them for production builds
    let config_path: String = format!("/home/{current_user}/Apps");
    let _ = fs::create_dir(&config_path);
    clear();
    println!("Found recommended architecture of {:?}", whoami::arch());
    let confirm_setup = prompt::confirm_prompt(String::from("Would you like to enter setup?"));
    match confirm_setup {
        Ok(true) => {
            
        }
        Ok(false) => return 1,
        Err(e) => {
            println!("Caught error {:?}", e)
        }
    }
    let input = prompt::create_password("Create a PIN to install packages", "Repeat the PIN");
    let confirm = prompt::confirm_prompt(String::from("Would you like to use a predefined package list?"));
    match confirm {
        Ok(val) => {
            if val {
                select_prompt(vec!["Minimal", "Development Tools", "Standard", "Complete"], String::from("Select a package base")).expect(":)");
            }
            if !val {
                println!("no")
            }
        },
        Err(_) => todo!(),
    }
    0
}