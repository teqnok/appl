use colored::{Colorize, ColoredString};
use walkdir::WalkDir;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
/// Common utilities for abstraction, like conversions and parsers.
use std::path::{Path, PathBuf};
use toml::Value;
use zip::read::ZipArchive;

use crate::pkgutils::{get_config, get_toml_keys};
use std::error::Error;

/// A complete rewrite of appl::install_package
pub fn get_pkg_info<T>(query: Vec<T>) -> Result<String, Box<dyn Error>> where T: ToString + PartialEq + Debug {
    let mut packages: Vec<PathBuf> = vec![];
 
    for pkg in WalkDir::new(get_config()) {
        // convert "~/.config/appl/pkg.toml" into pkg
        let pkgpath = pkg?.path().with_extension("");
        let rpkg = pkgpath.file_name().unwrap().to_str().unwrap().trim_matches('"');
        //---------------------------------------------
        for query in &query { 
            if query.to_string().trim_matches('"') == rpkg.to_string() {
                if !packages.contains(&pkgpath) {
                   packages.push(pkgpath.clone());
                }
            }
        }
    }
    for pkg in packages.clone() {
        let contents = read_pkg_file(pkg.to_str().unwrap(), "pkgdata.toml");
        let keys = get_toml_keys(contents.unwrap(), false)?;
        print_pkg_details(keys.clone(), true);

    } 
    println!("{packages:#?}");
    Ok("h".into())
 }
 
/// Searchs all packages for name matches.
pub fn pkg_search<T: ToString + PartialEq>(query: T) -> Result<String, Box<dyn std::error::Error>> {
    for pkg in WalkDir::new(get_config()) {
        let pkgpath = pkg?.path().with_extension("");
        let rpkg = pkgpath.file_name().unwrap().to_str().unwrap().trim_matches('"');
        if rpkg.contains(&query.to_string()) {
            
            // print_pkg_details(keys, false);
        }
    }
    Ok("h".into())
} 

/// Reads a compressed file from a .apkg archive (uses `zip`)
/// p: path of archive
/// f: filename to read
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
    let __name__ = v["name"].to_string();
    let package_name = __name__.trim_matches('"');
    let is_installed = if Path::new(&format!("{}{}", get_config(), package_name)).exists() {
        "[installed]".blue().bold()
    } else {
        "".white()
    };
    let package_version = v["version"].to_string().trim_matches('"').purple().bold();
    let arch = v.get("arch").map(|e| e.to_string().green().bold()).unwrap_or("X64".green().bold());
    let deps = dep_to_str(v);
    if deps.is_empty() {
        println!("{}{} {}-{}:{arch} {}", "=".blue(), ">".green(), package_name.purple().bold(), package_version, is_installed);
    } else {
        println!("{}{} {}-{}:{arch} {} {}{} (requires packages {})", "=".blue(), ">".green(), package_name.purple().bold(), package_version, is_installed, "==".blue(), ">".green(), deps.join(" "));
    }
 }
 