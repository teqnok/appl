use std::io::Error;

use colored::{ColoredString, Colorize};
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Password, Select};

pub fn select_prompt(items: Vec<&str>, prompt: String) -> std::io::Result<&str> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let mut select = "";
    match selection {
        Some(index) => {
            select = items[index]
        }
        None => println!("User did not select anything"),
    }

    Ok(select)
}
pub fn select_prompt_string(items: Vec<String>, prompt: String) -> std::io::Result<String> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let mut select = "";
    match selection {
        Some(index) => select = &items[index],
        None => println!("User did not select anything"),
    }

    Ok(select.to_string())
}

pub fn password_input(prompt_string: &str, confirmation: &str, password_to_confirm: &str) -> i32 {
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

use std::io::Write;
pub fn confirm_prompt_custom(display_text: String) -> Result<bool, Error> {
    let mut input: String = String::new();
    let mut confirmed: bool = false;

    print!(
        "{} [{}/{}] {} ",
        display_text,
        "y".green(),
        "n".red(),
        ">".blue()
    );
    print!(" ");
    std::io::stdout().flush().unwrap();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();
    input = input.trim().to_string();
    match input.as_str() {
        "y" => confirmed = true,
        "yes" => confirmed = true,
        "1" => confirmed = true,
        "n" => confirmed = false,
        "no" => confirmed = false,
        "0" => confirmed = false,
        _ => {}
    }

    Ok(confirmed)
}

pub fn prompt_input(display_text: ColoredString) -> String {
    let mut input: String = "".into();

    print!("{} {}", display_text, "> ".blue());

    std::io::stdout().flush().unwrap();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn int_input(display_text: ColoredString) -> i64 {
    let mut input_text = String::new();
    print!("{display_text} {}", ">".green());
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input_text).unwrap();

    match input_text.trim().parse::<i32>() {
        Ok(num) => num.into(),
        Err(e) => {
            println!("Failed to convert input to integer of type <i32>, returning 0.");
            println!("Exception: {e}");
            0
        }
    }
}