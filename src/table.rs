<<<<<<< HEAD
use crate::config::{get_config_table, get_config_value};
use crate::package::Package;
use colored::Colorize;
use stanza::renderer::console::Console;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, MaxWidth, MinWidth, Palette16, Styles, TextFg};
use stanza::table::{Cell, Col, Row, Table};
pub fn install_search(pkgs: Vec<Package>) {
=======
use crate::config::get_config_value;
use crate::package::Package;
use stanza::renderer::console::Console;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, MinWidth, Palette16, Styles, TextFg};
use stanza::table::{Cell, Col, Row, Table};
pub fn render_package_table(pkgs: Vec<Package>) {
>>>>>>> 90a997964089276d576c30b1c142cf1ab0495143
    let mut table = Table::default()
        .with_cols(vec![
            Col::new(Styles::default().with(MinWidth(12)).with(HAlign::Left)),
            Col::new(Styles::default().with(MinWidth(12)).with(HAlign::Left)),
            Col::new(Styles::default().with(MinWidth(12)).with(HAlign::Left)),
            Col::new(Styles::default().with(MinWidth(12)).with(HAlign::Left)),
        ])
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec![
                "Repo".into(),
                "Package".into(),
                "New Version".into(),
                "Download Size".into(),
            ],
        ));
    table.push_rows(build_body_rows(pkgs));
    if get_config_value("config", "prefer_md").unwrap() == "true" {
        let renderer = Markdown::default();
        println!("{}", renderer.render(&table));
    } else {
        let renderer = Console::default();
        println!("{}", renderer.render(&table));
    }
}
fn build_body_rows(pkgs: Vec<Package>) -> Vec<Row> {
    (1..=pkgs.len())
        .map(|row| {
            let mut cells = vec![];
            let repo = pkgs.iter().nth(row - 1).unwrap().repo.replace('"', "");
            let repo_cell = Cell::new(
                Styles::default().with(TextFg(Palette16::BrightCyan)),
                repo.into(),
            );
            cells.push(repo_cell);

            let name = pkgs.get(row - 1).unwrap().name.replace('"', "").to_string();
            let name_cell = Cell::new(
                Styles::default()
                    .with(TextFg(Palette16::BrightWhite))
                    .with(HAlign::Left),
                name.into(),
            );
            cells.push(name_cell);

            let version = pkgs.get(row - 1).unwrap().version.replace('"', "");
            let version_cell = Cell::new(
                Styles::default().with(TextFg(Palette16::BrightGreen)),
                version.into(),
            );
            cells.push(version_cell);

<<<<<<< HEAD
            let mut download = pkgs
=======
            let download = pkgs
>>>>>>> 90a997964089276d576c30b1c142cf1ab0495143
                .get(row - 1)
                .unwrap()
                .download
                .to_string()
                .replace('"', "");
<<<<<<< HEAD
            download.push_str(" MiB");
=======
>>>>>>> 90a997964089276d576c30b1c142cf1ab0495143
            let download_cell = Cell::new(
                Styles::default()
                    .with(TextFg(Palette16::Blue))
                    .with(HAlign::Left),
                download.into(),
            );
            cells.push(download_cell);

            Row::new(Styles::default(), cells)
        })
        .collect()
}
<<<<<<< HEAD
pub fn rough_search(pkgs: Vec<Package>) {
    for package in pkgs {
        println!(
            "{}{}{} {} {}",
            package.repo.blue(),
            "::".bold(),
            package.name.green(),
            "|".bold(),
            package.version.bold().blue()
        );
        println!("\t {}", package.desc.bold());
    }
}
=======
>>>>>>> 90a997964089276d576c30b1c142cf1ab0495143
