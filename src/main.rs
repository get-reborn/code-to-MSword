use clap::Parser;
use filter::FileFilter;
mod cli;
mod filter;

fn main() {
    let cli = cli::Cli::parse();
    let ff = FileFilter::new(&cli);
    ff.run();
}
