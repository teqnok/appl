use std::io::Error;

use dialoguer::{Password, theme::ColorfulTheme, Confirm, console::Term, Select};

pub fn select_prompt(items: Vec<&str>, prompt: String) -> std::io::Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything")
    }

    Ok(())
}

pub fn password_input(prompt_string: &str,confirmation: &str, password_to_confirm: &str) -> i32 {
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_string)
        .with_confirmation(confirmation, "Error: the passwords don't match.")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.chars().count() > 3 {
                Ok(())
            } else {
                Err("Password must be longer than 3")
            }
        })
        .interact()
        .unwrap();

    if password == password_to_confirm {
        0
    } else {
        1
    }
    
}
pub fn create_password(prompt_string: &str, confirmation: &str) -> String {
    let password = Password::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt_string)
    .with_confirmation(confirmation, "Error: the passwords don't match.")
    .validate_with(|input: &String| -> Result<(), &str> {
        if input.chars().count() > 3 {
            Ok(())
        } else {
            Err("Password must be longer than 3")
        }
    })
    .interact()
    .unwrap();
    password
}

pub fn confirm_prompt(display_text: String) -> Result<bool, Error> {
    if match Confirm::with_theme(&ColorfulTheme::default()).with_prompt(display_text).default(true).show_default(true).interact() {
        Ok(it) => it,
        Err(err) => return Err(err),
    } {
        return Ok(true);
    } else {
        return Ok(false)
    }
    
}
