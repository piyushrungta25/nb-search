extern crate clap;

use std::{env};
use clap::{Arg, App};
use nb_search;

fn main() {
    let cwd = env::current_dir().unwrap();
    let cwd = cwd.to_str().unwrap();

    let matches = App::new("nbs")
                          .version("0.1")
                          .author("Piyush Rungta <piyushrungta25@gmail.com>")
                          .arg(Arg::with_name("pattern")
                              .help("Pattern to search for")
                              .value_name("PATTERN")
                              .required(true)
                              .index(1))
                          .arg(Arg::with_name("paths")
                              .help("Paths to search for, defaults to PWD if not specified")
                              .required(false)
                              .value_name("PATHS")
                              .multiple(true)
                              .index(2)
                              .default_value(cwd)
                              .hide_default_value(true))
                          .arg(Arg::with_name("filename")
                              .short("H")
                              .long("with-filename")
                              .help("Print file names before each matched notebook"))
                          .get_matches();
    
    nb_search::search(matches);

    // println!("{:#?}", matches);
}
