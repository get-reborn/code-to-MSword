use clap::Parser;

use exporter::Exporter;
use filter::FileFilter;
use reader::Reader;

mod cli;
mod exporter;
mod filter;
mod reader;

fn main() {
    let cli = cli::Cli::parse();
    let ff = FileFilter::new(&cli);
    let file_array = ff.run();
    let read_string = Reader::read(&file_array);
    Exporter::export(&cli.output_path, &cli.output_file_name, &read_string);
}
