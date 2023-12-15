use crate::pkgutils::{get_config, get_toml_keys};
use crate::prompt::confirm_prompt_custom;
use colored::{ColoredString, Colorize};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};
use std::time::Instant;
use toml::Value;
use walkdir::WalkDir;
use zip::read::ZipArchive;
/// A rewrite of `appl::install_package()` that is much more concise and efficent.
pub fn get_pkg_info<T>(query: Vec<T>) -> Result<String, Box<dyn Error>>
where
    T: ToString + PartialEq + Debug,
{
    let time = Instant::now();

    let packages: Vec<PathBuf> = WalkDir::new(get_config())
        .into_iter()
        .filter_map(|pkg| {
            let pkgpath = pkg.ok()?.path().to_path_buf();
            let extensionless = pkgpath.with_extension("");
            let rpkg = extensionless.file_name()?.to_str()?.trim_matches('"');

            query
                .iter()
                .find(|q| q.to_string().trim_matches('"') == rpkg.to_string())
                .map(|_| pkgpath)
        })
        .collect();

    if !packages.is_empty() {
        let (mut download_size, mut install_size) = (0, 0);
        println!(
            "Found {} packages \n\nPackages to install: \n",
            packages.len().to_string().green()
        );
        for pkg in packages.clone() {
            let contents = read_pkg_file(pkg.to_str().unwrap(), "package.toml");
            let keys = get_toml_keys(contents?.to_string(), false)?;
            print_pkg_details(keys.clone(), ApplOperation::Install);
            download_size += size_to_str(keys.clone(), true);
            install_size += size_to_str(keys, false);
        }
        println!();
        println!(
            "Download size: {} MiB {} Install size: {} MiB [took {:?}]",
            download_size.to_string().blue().bold(),
            "|".bold().dimmed().cyan(),
            install_size.to_string().green().bold(),
            time.elapsed()
        );
        let confirm = confirm_prompt_custom("Install these packages?".into())?;
        if confirm {

        } else {}
    } else {
        println!("No matches found for terms {:?}", query);
    }

    Ok("h".into())
}

/// Reads a compressed file from a zip-compressed archive (uses `zip`). Extension should not be relevant
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

/// Searchs all packages for name matches.
/// TODO (2) make this work
pub fn pkg_search<T: ToString + PartialEq>(query: T) -> Result<String, Box<dyn std::error::Error>> {
    for pkg in WalkDir::new(get_config()) {
        let pkgpath = pkg?.path().with_extension("");
        let rpkg = pkgpath
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_matches('"');
        if rpkg.contains(&query.to_string()) {

            // print_pkg_details(keys, ApplOperation::GetInfo);
        }
    }
    Ok("h".into())
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
/// Represents a user-invoked command.
pub enum ApplOperation {
    Install,
    Remove,
    Upgrade,
    GetInfo,
}

/// v: Package keys (toml::Value)
/// o: What operation is being performed
pub fn print_pkg_details(v: Value, o: ApplOperation) {
    let __name__ = v["name"].to_string();
    let package_name = __name__.trim_matches('"');
    let is_installed = if Path::new(&format!("{}{}", get_config(), package_name)).exists() {
        "[installed]".blue().bold()
    } else {
        "".white()
    };
    let package_version: ColoredString = v["version"].to_string().trim_matches('"').purple().bold();
    let arch: ColoredString = v
        .get("arch")
        .map(|e| e.to_string().green())
        .unwrap_or("X64".green())
        .trim_matches('"')
        .green();
    let desc: String = v
        .get("description")
        .map(|e| e.to_string())
        .unwrap_or("".into());
    let deps: Vec<String> = dep_to_str(v);
    match o {
        ApplOperation::Install | ApplOperation::Remove | ApplOperation::Upgrade => {
            if deps.is_empty() {
                println!(
                    "{}{} {}-{}:{arch} {}",
                    "=".blue(),
                    ">".green(),
                    package_name.purple().bold(),
                    package_version,
                    is_installed
                );
            } else {
                println!(
                    "{}{} {}-{package_version}:{arch} {} {}{} (requires packages {})",
                    "=".blue(),
                    ">".green(),
                    package_name.purple().bold(),
                    is_installed,
                    "==".blue(),
                    ">".green(),
                    deps.join(" ")
                );
            }
        }
        ApplOperation::GetInfo => {
            println!(
                "{}{} {} \n {}{} {}",
                "=".blue(),
                ">".green(),
                package_name.green().bold(),
                "==".blue(),
                ">".green(),
                desc
            );
        }
    }
}
