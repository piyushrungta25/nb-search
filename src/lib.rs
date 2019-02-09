#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate clap;

use regex::Regex;
use walkdir::{WalkDir, DirEntry};
use std::ffi::OsStr;
use std::fs::File;
// use serde_json::{Value};

use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Cell {
    source: Vec<String>,
    cell_type: String
}

#[derive(Serialize, Deserialize)]
struct Notebook {
    cells: Vec<Cell>
}

fn is_notebook(entry: &DirEntry) -> bool {
    let is_file:bool = entry.file_type().is_file();

    // If for a directory, false is returned, then that directory is not recursed
    // into. This block just returns true for a directory if not hidden.
    if !is_file {
        let is_hidden = entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false);
        return !is_hidden;
    }

    let ext = entry.path().extension();
    let ext = ext.unwrap_or(OsStr::new("")).to_str().unwrap();
    let ext: bool = ext == "ipynb";
    ext
}


pub fn search(args: clap::ArgMatches) {
    let pattern = args.value_of("pattern").unwrap();
    println!("matchin: {}", pattern);
    let re = Regex::new(pattern).unwrap();
    for path in args.values_of("paths").unwrap() {
        for entry in WalkDir::new(path).into_iter().filter_entry(is_notebook) {
            let e = entry.unwrap(); 
            if e.file_type().is_file() {
                let fpath = e.path();
                println!("{}", fpath.to_str().unwrap());
                println!("");
                let fp = File::open(fpath).unwrap();
                let v: Notebook = serde_json::from_reader(fp).unwrap();
                for (i, cell) in v.cells.iter().enumerate() {
                    if cell.cell_type == "code" && cell.source.len() > 0 {
                        let mut cell_no_printed = false;

                        for ln in cell.source.iter() {
                            if re.is_match(ln) {
                                if !cell_no_printed {
                                    println!("  Cell {}:", i);
                                    cell_no_printed = true;
                                }
                                println!("    {}", ln.trim_end_matches('\n'));
                            }
                        }
                        if cell_no_printed {
                            println!("");
                        }
                    }
                }
            }
        }
    }


}
