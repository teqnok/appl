use dialoguer::{Password, theme::ColorfulTheme};

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
