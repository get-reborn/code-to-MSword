use clap::Parser;
use filter::FileFilter;
use reader::Reader;
mod cli;
mod filter;
mod reader;

fn main() {
    let cli = cli::Cli::parse();
    let ff = FileFilter::new(&cli);
    let file_array = ff.run();
    let read_string = Reader::read(&file_array);
}
