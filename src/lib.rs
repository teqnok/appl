// Library functions for appl
// This API aims to provide the following:
// - functions to create a package out of a inputted file
// -  mod prompt;

use std::process::Command;

pub fn clear() {
    assert!( std::process::Command::new("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success() );
}
