use clap::Parser;

pub mod csv_opts;
pub mod genpass_opts;

use csv_opts::CsvOpts;
use genpass_opts::GenPassOpts;

#[derive(Parser, Debug)]
#[command(name = "csv", version, about, author, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    // #[command(name = "base64", about = "Encode or Decode Base64")]
    // Base64(Base64SubCommand),
}
