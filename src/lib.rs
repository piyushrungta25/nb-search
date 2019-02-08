extern crate clap;
extern crate grep;

use walkdir::{WalkDir, DirEntry};
use std::ffi::OsStr;
use std::fs::File;
use serde_json::{Value};
use grep::{searcher};
use grep::searcher::sinks::UTF8;
use grep::regex::RegexMatcher;



fn is_notebook(entry: &DirEntry) -> bool {
    let is_file:bool = entry.file_type().is_file();
    if !is_file {
        
        let is_hidden = entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false);
        
        return !is_hidden;
    }

    let ext = entry.path().extension();
    // if ext.is_some() {
    let ext = ext.unwrap_or(OsStr::new("")).to_str().unwrap();
    // println!("found {} for {}", ext, entry.path().display());
    let ext: bool = ext == "ipynb";
    // }

    // let () = ext;
    ext
    // false
}


pub fn search(args: clap::ArgMatches) {
    // println!("{}", args.values_of("paths").unwrap());
    // let walk = Walker{paths: args.values_of("paths").unwrap()}
    for path in args.values_of("paths").unwrap() {

        for entry in WalkDir::new(path).into_iter().filter_entry(is_notebook) {
            // let () = entry;
            let e = entry.unwrap(); 
            if e.file_type().is_file() {
                let fpath = e.path();
                let fp = File::open(fpath).unwrap();
                let v: Value = serde_json::from_reader(fp).unwrap();
                let sc = v["worksheets"][0]["cells"][0]["source"][0].as_str().unwrap_or("").as_bytes();
                let matcher = RegexMatcher::new(r"the").unwrap();
                // let mut matches: Vec<(u64, String)> = vec![];
                let _ = searcher::Searcher::new().search_slice(&matcher, sc, UTF8(|_lnum, line| {
                    // We are guaranteed to find a match, so the unwrap is OK.
                    // let mymatch = matcher.find(line.as_bytes())?.unwrap();
                    // matches.push((lnum, line[mymatch].to_string()));
                    println!("{}", line);
                    Ok(true)
                }));
                // println!("{}", v["worksheets"][0]["cells"][0]["source"][0].as_str().unwrap());
                // println!("");
                // println!("=========================");
                // println!("=========================");
                // println!("");
                
            }
        }
    }


}
