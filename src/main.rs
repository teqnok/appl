pub mod cli;
pub mod cmd;
pub mod config;
pub mod db;
pub mod input;
pub mod package;
pub mod table;
#[derive(Clone, Debug)]
pub struct ApplInstance {
    repos: Vec<db::Repo>,
}
impl ApplInstance {
    pub fn search_exact<T: ToString>(self, term: T) -> Vec<package::Package> {
        let mut packages = Vec::new();
        for repo in self.repos {
            packages.extend(repo.search_exact(term.to_string()))
        }
        packages
    }
    pub fn search<T: ToString>(self, term: T) -> Vec<package::Package> {
        let mut packages = Vec::new();
        for repo in self.repos {
            packages.extend(repo.search(term.to_string()))
        }
        packages
    }
}
fn main() {
    let appl = ApplInstance {
        repos: db::Repo::init(),
    };
    cmd::builder(appl);
}
