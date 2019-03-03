# nb-search

This is a simple tool written in rust to search [jupyter notebooks](https://jupyter.org/) with [ack](https://beyondgrep.com/) like interface. This is still a work in progress. New features coming soon!!

Uses serde-json to parse notebooks, clap for command line arguments, regex for actual matching and termcolor for colorized outputs.


## Usage

```bash
nbs [FLAGS] <PATTERN> [PATHS]
```
If PATHS not specified, search all the notebooks in current directory.

## Features
- Searches only code cells of the notebook notebook
- Colored output

## Planned Features
- Detect tty and don't emit color codes
- More grep/ack/ripgrep like flages like printing context, no-color etc.
- Optional searching markdown and output cells



