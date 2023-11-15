
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn script_viewer(script: String) {
    let path = Path::new(script.as_str());
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in reader.lines() {
        let line = line.unwrap();
        let ranges: Vec<(Style, &str)> = h.highlight_line(&line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
     }
    
}