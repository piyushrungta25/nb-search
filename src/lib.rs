#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate clap;
extern crate termcolor;

use regex::Regex;
use walkdir::{WalkDir, DirEntry};
use std::ffi::OsStr;
use std::fs::File;
use std::process;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Cell {
    source: Vec<String>,
    cell_type: String
}

#[derive(Serialize, Deserialize)]
struct Notebook {
    cells: Vec<Cell>
}

// Check if the entry is a file and if has the extension ".ipynb"
fn is_notebook(entry: Result<walkdir::DirEntry, walkdir::Error>) -> Option<walkdir::DirEntry> {
    let entry = entry.unwrap();

    if entry.file_type().is_file() {
        let ext = entry.path().extension();
        let ext = ext.unwrap_or(OsStr::new("")).to_str().unwrap();
        if ext == "ipynb" {
            return Some(entry);
        }
    }
    return None;
}

fn print_highlighted_code(matches: Vec<regex::Match>, ln: &str) {
    let mut init = 0;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.reset().unwrap();
    print!("    ");
    for m in matches.into_iter() {
        stdout.reset().unwrap();
        print!("{}", &ln[init..m.start()]);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
        print!("{}", m.as_str());

        init = m.end();
    }
    stdout.reset().unwrap();
    print!("{}", &ln[init..]);
    

    println!("");

}

fn search_and_print(e: walkdir::DirEntry, re: &regex::Regex) {
    let fpath = e.path();
    let mut file_path_printed = false;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let fp = File::open(fpath).unwrap();
    let v: Notebook = serde_json::from_reader(fp).unwrap();
    for (i, cell) in v.cells.iter().enumerate() {
        if cell.cell_type == "code" && cell.source.len() > 0 {
            let mut cell_no_printed = false;

            for ln in cell.source.iter() {
                let ln = ln.trim_end_matches('\n');
                let matches: Vec<regex::Match> = re.find_iter(ln).collect();

                if matches.len() > 0 {
                    if !file_path_printed {
                        file_path_printed = true;
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
                        writeln!(&mut stdout, "{}", fpath.to_str().unwrap()).unwrap();
                    }
                    if !cell_no_printed {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                        println!("  Cell {}:", i);
                        cell_no_printed = true;
                    }
                    stdout.reset().unwrap();
                    print_highlighted_code(matches, ln);
                }
            }
            if cell_no_printed {
                println!("");
            }
        }
    }
}

pub fn search(args: clap::ArgMatches) {
    let pattern = args.value_of("pattern").unwrap();
    println!("matchin: {}", pattern);
    let re = Regex::new(pattern).unwrap();
    for path in args.values_of("paths").unwrap() {
        for entry in WalkDir::new(path).into_iter().filter_map(is_notebook) {
            search_and_print(entry, &re);
        }
    }
}
