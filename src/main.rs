use clap::Parser;

use filter::FileFilter;
use reader::Reader;
use exporter::Exporter;

mod cli;
mod filter;
mod reader;
mod exporter;

fn main() {
    let cli = cli::Cli::parse();
    let ff = FileFilter::new(&cli);
    let file_array = ff.run();
    let read_string = Reader::read(&file_array);
    Exporter::export(&cli.output_path, &cli.output_file_name, &read_string);
}
