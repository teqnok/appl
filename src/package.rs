use mlua::prelude::*;
use serde::{Deserialize, Serialize};
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
    /// Backend silent function for installing a package.
    pub fn install(&self) -> u32 {
        let package_script = crate::config::get_appl_dir("scripts/").unwrap();
        let pscript = format!("{}{}/{}.lua", package_script, self.repo, self.name);
        let contents = std::fs::read_to_string(&pscript).unwrap();
        let lua = Lua::new();
        lua.create_table().unwrap();
        let globals = lua.globals();
        globals.set("pkgname", self.name.clone()).unwrap();
        globals.set("pkgver", self.version.clone()).unwrap();
        lua.load(contents).exec().unwrap();
        1
    }
}
