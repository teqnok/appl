pub mod cli;
pub mod cmd;
pub mod config;
pub mod db;
pub mod input;
<<<<<<< HEAD
pub mod log;
pub mod package;
pub mod setup;
=======
pub mod package;
>>>>>>> 90a997964089276d576c30b1c142cf1ab0495143
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
<<<<<<< HEAD
#[tokio::main]
async fn main() {
    let appl = ApplInstance {
        repos: db::Repo::init(),
    };
    cmd::builder(appl).await;
=======
fn main() {
    let appl = ApplInstance {
        repos: db::Repo::init(),
    };
    cmd::builder(appl);
>>>>>>> 90a997964089276d576c30b1c142cf1ab0495143
}
