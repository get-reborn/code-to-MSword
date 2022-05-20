use clap::Parser;
use std::env::current_dir;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Path of directory contains code files
    #[clap(short, long, default_value_t = current_dir().unwrap().to_str().unwrap().to_string())]
    pub path: String,

    /// String to match code files
    #[clap(short, long, default_value_t = String::new())]
    pub match_str: String,

    /// Excluse directories or files
    #[clap(short, long)]
    pub excluse_strs: Vec<String>,
}
