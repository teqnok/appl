use colored::Colorize;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{prelude::*, Error};
/// Common utilities for abstraction, like conversions and parsers.
use std::path::{Path, PathBuf};
use toml::Value;
use zip::read::ZipArchive;

use crate::pkgutils::get_config;



pub fn get_pkg_info<T: ToString + PartialEq>(query: Vec<T>) -> Result<String, Error> {
    let mut packages: Vec<PathBuf> = vec![];

    for pkg in WalkDir::new(get_config()) {
        let pkgpath = pkg?.path().with_extension("");
        let rpkg = pkgpath.file_name().unwrap().to_str().unwrap().trim_matches('"');
        for query in &query {
            if query.to_string() == rpkg.to_string() {
                packages.push(pkgpath.clone());
            }
        }
    }
    println!("{packages:#?}");
    Ok("h".into())
}

/// Reads a compressed file from a .apkg archive (uses `zip`)
// p: path of archive
// f: filename to read
pub fn read_pkg_file(p: &str, f: &str) -> zip::result::ZipResult<String> {
    let reader = File::open(p)?;
    let mut zip = ZipArchive::new(reader)?;
    let mut retstr: String = "".into();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name() == f {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            retstr = contents;
        }
    }
    Ok(retstr)
}

//------------------------------------------------------
/// Read dependencies into a vector
pub fn dep_to_str(a: Value) -> Vec<String> {
    let p: Vec<String> = a["dependencies"]
        .as_array()
        .expect(&"Dependencies not found!".yellow())
        .iter()
        .map(|s| s.to_string())
        .collect();
    p
}
/// Read download/install size into int
pub fn size_to_str(a: Value, is_dsize: bool) -> i64 {
    let mut str = "install_size";
    if is_dsize {
        str = "download_size"
    }
    let int = a.get(str).unwrap().as_integer().unwrap();
    int
}
//---------------------------------------------------------------------------
/// v: Package keys
/// i: has dependencies
/// ii
pub fn print_pkg_details(v: Value, i: bool) {
    let package_name = Path::new(v["name"].as_str().unwrap())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let mut is_installed = "";
    if i {
        is_installed = "[installed]"
    }
    let package_version = v["version"].to_string().trim_matches('"').purple().bold();
    if i {
        println!(
            "{}{} {}-{}:{} [{}] {} \n",
            "=".blue(),
            ">".green(),
            package_name.trim_matches('"').purple().bold(),
            format!("{package_version}"),
            v["arch"].to_string().trim_matches('"').green().bold(),
            v["branch"].to_string().trim_matches('"').green().bold(),
            is_installed,
        )
    } else {
        println!(
            "{}{} {}-{}:{} [{}] {} \n{}{} (requires packages {}) \n",
            "=".blue(),
            ">".green(),
            package_name.trim_matches('"').purple().bold(),
            format!("{package_version}"),
            v["branch"].to_string().trim_matches('"').green().bold(),
            v["arch"].to_string().trim_matches('"').green().bold(),
            is_installed,
            "==".blue(),
            ">".green(),
            dep_to_str(v.clone()).join(" "),
        );
    }
}
