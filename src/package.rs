use anyhow::Result;
use colored::ColoredString;
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use mlua::prelude::*;
use reqwest::header::CONTENT_LENGTH;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{io::Write, path::PathBuf};

use crate::log;

pub async fn download_file(
    url: &str,
    path: &str,
    name: ColoredString,
) -> Result<(), anyhow::Error> {
    // Remove quotes from the string ("string" -> string)
    let url = url.trim_matches('"');
    let mut name = name.trim_matches('"').to_string();
    // Create a reqwest client
    let client = reqwest::Client::new();
    let new_path = PathBuf::from(path);
    // Send a GET request to the file URL
    let response = client.get(url).send().await?;
    // Get the total size of the file from the Content-Length header
    let total_size = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0);
    // If there is no Content-Length (GitHub doesn't provide one sometimes), indicatif will report a size of 0.
    // TODO see if fixable
    // Align the progress bar with the package name, for prettiness. If the package name is longer
    if name.len() >= 15 {
        name.truncate(12);
        name.push_str("...");
    }
    let mut spaces = " ".repeat(15);
    for _ in 1..=name.len() {
        if spaces.len() == 0 {
            break;
        }
        spaces.remove(0);
    }
    // Create a progress bar with the total size of the file
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{msg}[{elapsed_precise}] [{bar:25.cyan/blue}] {bytes}/{total_bytes}",
        )
        .unwrap()
        .progress_chars("󰹞 "),
    );
    pb.set_message(format!(
        "{} {}{}",
        "=>".green(),
        name.bold().green(),
        spaces
    ));
    // Open the file in write-only mode
    std::fs::create_dir_all(new_path.parent().unwrap())?;
    let mut file = std::fs::File::create(path)?;

    // Read the response body in chunks
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        // Write the chunk to the file
        file.write_all(&chunk)?;
        // Update the progress bar with the length of the chunk
        pb.inc(chunk.len() as u64);
    }
    // Finish the progress bar
    pb.finish();
    println!("h");

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub desc: String,
    pub homepage: String,
    pub license: String,
    pub authors: Vec<String>,
    pub repo: String,
    pub download: u32,
    pub depends: Vec<String>,
}

impl Package {
    /// Backend function for installing a package.
    pub async fn install(self) -> Result<(), LuaError> {
        let package_script = crate::config::get_appl_dir("scripts/").unwrap();
        let pscript = format!("{}{}/{}.lua", package_script, self.repo, self.name);
        let contents = std::fs::read_to_string(&pscript).unwrap();
        let lua = Lua::new();
        let globals = lua.globals();

        globals.set("pkgname", self.name.clone())?;
        globals.set("pkgver", self.version.clone())?;
        let exec = lua.create_function(move |_, (path, args): (String, Vec<String>)| {
            Command::new(path).args(args).spawn()?;
            Ok(())
        })?;
        globals.set("exec", exec)?;

        lua.load(&contents).exec()?;
        handle_sources(globals.get::<_, Vec<String>>("sources")?).await;
        println!("{}", "==> Building package...".green());
        lua.load("build()").exec()?;
        Ok(())
    }
    pub async fn run(self) -> Result<(), LuaError> {
        let package_script = crate::config::get_appl_dir("scripts/").unwrap();
        let pscript = format!("{}{}/{}.lua", package_script, self.repo, self.name);
        let contents = std::fs::read_to_string(&pscript).unwrap();
        let lua = Lua::new();
        let globals = lua.globals();
        globals.set("pkgname", self.name.clone())?;
        globals.set("pkgver", self.version.clone())?;
        let exec = lua.create_function(move |_, (path, args): (String, Vec<String>)| {
            Command::new(path).args(args).spawn()?.wait_with_output();
            Ok(())
        })?;
        
        globals.set("exec", exec)?;

        lua.load(&contents).exec()?;
        lua.load("run()").exec()?;
        Ok(())
    }
}
async fn handle_sources(sources: Vec<String>) {
    for source in sources {
        let path = PathBuf::from(&source);
        let name = path.file_name().unwrap().to_str().unwrap();
        download_file(&source, name, name.into()).await;
    }
}
