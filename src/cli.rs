use crate::entry_file::EntryFile;
use clap::Parser;
use owo_colors::OwoColorize;
use std::fs::{self};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, long_about = "ls but with a different skin")]
pub struct Cli {
    pub path: Option<PathBuf>,
    ///Prints entries with json format
    #[arg(short, long)]
    pub json: bool,
    ///Include the entries that start with "."
    #[arg(short, long)]
    pub all: bool,
    ///Include the modified date
    #[arg(short, long)]
    pub modified: bool,
}

impl Cli {
    pub fn lsr(cli: Cli) {
        let path = cli.path.unwrap_or(PathBuf::from("."));

        if let Ok(does_exist) = fs::exists(&path) {
            if does_exist {
                if cli.json {
                    let files = EntryFile::get_files(&path, cli.all);
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&files)
                            .unwrap_or("cannot parse json".to_string())
                    );
                } else {
                    EntryFile::print_table(&path, cli.all, cli.modified);
                }
            } else {
                println!("{}", "Path doesn't exists".red());
            }
        } else {
            println!("{}", "Error reading the directory".red());
        }
    }
}
