use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::Select;
pub fn confirm_prompt<T: ToString>(prompt: T) -> bool {
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.to_string())
        .interact_opt();
    match confirm {
        Ok(confirmed) => confirmed.unwrap(),
        Err(_) => false,
    }
}
pub fn confirm_prompt_simple<T: ToString>(prompt: T) -> bool {
    let confirm = Confirm::new()
        .with_prompt(prompt.to_string())
        .interact_opt();
    match confirm {
        Ok(confirmed) => confirmed.unwrap(),
        Err(_) => false,
    }
}

pub fn input_prompt<T: ToString + Clone>(prompt: T, default: T) -> String {
    let input = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.to_string())
        .default(default.to_string())
        .interact()
        .unwrap();
    input
}

pub fn select_prompt<T: ToString + Clone>(prompt: T, options: Vec<T>) -> T {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.to_string())
        .items(&options)
        .default(0)
        .interact_opt();
    match selection {
        Ok(selection) => options[selection.unwrap()].clone(),
        Err(_) => options[0].clone(),
    }
}
pub fn select_prompt_simple<T: ToString + Clone>(prompt: T, options: Vec<T>) -> T {
    let selection = Select::new()
        .with_prompt(prompt.to_string())
        .items(&options)
        .default(0)
        .interact_opt();
    match selection {
        Ok(selection) => options[selection.unwrap()].clone(),
        Err(_) => options[0].clone(),
    }
}
