use crate::package::Package;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repo {
    pub name: String,
    pub packages: Vec<crate::package::Package>,
}
impl Repo {
    pub fn init() -> Vec<Self> {
        let mut repos = Vec::new();
        let loaded_repos = PathBuf::from(crate::config::get_appl_dir("repos/").unwrap());
        for entry in fs::read_dir(loaded_repos).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_file() {
                println!("Invalid file in repos directory: {}", path.display());
            }
            let mut file = fs::File::open(&path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let repo: Repo = toml::from_str(&contents).unwrap();
            repos.push(repo);
        }
        repos
    }
    /// This is for installing, not searching packages. Use Repo::search instead for searching.
    pub fn search_exact<T: ToString>(self, term: T) -> Vec<Package> {
        let mut packages = Vec::new();
        for package in self.packages {
            if package.name == term.to_string() {
                packages.push(package)
            }
        }
        packages
    }

    /// This is for searching, not installing packages. Use Repo::search_exact instead for
    /// installing.
    pub fn search<T: ToString>(self, term: T) -> Vec<Package> {
        let mut packages = Vec::new();
        for package in self.packages {
            if package.name.contains(&term.to_string()) {
                packages.push(package)
            }
        }
        packages
    }
}
